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
use std::env;

use containerd_shim::publisher::RemotePublisher;
use containerd_shim_protos::{events::TaskOom, prost_types::Any};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Must not start with unix://
    let address = args
        .get(1)
        .ok_or("First argument must be containerd's TTRPC address to publish events")
        .unwrap();

    println!("Connecting: {}", &address);

    let publisher = RemotePublisher::new(address).await.expect("Connect failed");

    let event = TaskOom {
        container_id: "123".into()
    };

    println!("Sending event");

    publisher
        .publish("/tasks/oom", "default", Any::from_msg(&event).unwrap())
        .await
        .expect("Publish failed");

    println!("Done");
}
