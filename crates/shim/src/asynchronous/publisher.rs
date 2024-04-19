/*
   Copyright The containerd Authors.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use containerd_shim_protos::{
    prost_types::Any,
    shim::events::{self, Events},
    trapeze::{self, Client},
};

use crate::{error::Result, util::timestamp};

/// Async Remote publisher connects to containerd's TTRPC endpoint to publish events from shim.
pub struct RemotePublisher {
    client: Client,
}

impl RemotePublisher {
    /// Connect to containerd's TTRPC endpoint asynchronously.
    ///
    /// containerd uses `/run/containerd/containerd.sock.ttrpc` by default
    pub async fn new(address: impl AsRef<str>) -> Result<RemotePublisher> {
        let client = Self::connect(address).await?;
        Ok(RemotePublisher { client })
    }

    async fn connect(address: impl AsRef<str>) -> Result<Client> {
        let address = address.as_ref();
        Client::connect(address)
            .await
            .map_err(io_error!(err, "Connecting to {address}"))
    }

    /// Publish a new event.
    ///
    /// Event object can be anything that Protobuf able serialize (e.g. implement `Message` trait).
    pub async fn publish(&self, topic: &str, namespace: &str, event: Any) -> Result<()> {
        let req = events::ForwardRequest {
            envelope: events::Envelope {
                topic: topic.into(),
                timestamp: timestamp()?.into(),
                namespace: namespace.into(),
                event: event.into(),
            }
            .into(),
        };

        self.client.forward(req).await?;

        Ok(())
    }
}

impl Events for RemotePublisher {
    async fn forward(&self, req: events::ForwardRequest) -> trapeze::Result<()> {
        self.client.forward(req).await
    }
}

#[cfg(test)]
mod tests {
    use containerd_shim_protos::{api::ForwardRequest, events::TaskOom, trapeze::{service, ServerConnection}};
    use tokio::sync::mpsc::{channel, Sender};
    use trapeze::transport::Listener as _;

    use super::*;

    struct FakeServer {
        tx: Sender<i32>,
    }

    impl Events for FakeServer {
        async fn forward(&self, req: ForwardRequest) -> trapeze::Result<()> {
            let env = req.envelope.unwrap_or_default();
            if env.topic == "/tasks/oom" {
                self.tx.send(0).await.unwrap();
            } else {
                self.tx.send(-1).await.unwrap();
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_connect() {
        let tmpdir = tempfile::tempdir().unwrap();
        let addr = format!("unix://{}/socket", tmpdir.as_ref().to_str().unwrap());

        assert!(RemotePublisher::connect("a".repeat(16384)).await.is_err());
        assert!(RemotePublisher::connect(&addr).await.is_err());

        let (tx, mut rx) = channel(1);
        let mut listener = trapeze::transport::bind(&addr).await.unwrap();
        tokio::spawn(async move {
            let conn = listener.accept().await.unwrap();
            ServerConnection::new(conn)
                .register(service!(FakeServer { tx } : Events))
                .start()
                .await
        });

        let client = RemotePublisher::new(&addr).await.unwrap();
        let msg = TaskOom {
            container_id: "test".into(),
        };
        client.publish("/tasks/oom", "ns1", Any::from_msg(&msg).unwrap()).await.unwrap();

        match rx.recv().await {
            Some(0) => {}
            _ => {
                panic!("the received event is not same as published")
            }
        }
    }
}
