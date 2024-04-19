// Copyright (c) 2019 Ant Financial
// Copyright (c) 2021 Ant Group
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use containerd_shim_protos::api::{CreateTaskRequest, CreateTaskResponse};
use containerd_shim_protos::Task;
use tokio::signal::ctrl_c;
use trapeze::{get_context, service, Result, Server};

#[derive(Debug, PartialEq)]
struct FakeServer {
    magic: u32,
}

impl FakeServer {
    fn new() -> Self {
        FakeServer { magic: 0xadcbdacf }
    }
}

impl Task for FakeServer {
    async fn create(&self, req: CreateTaskRequest) -> Result<CreateTaskResponse> {
        let ctx = get_context();
        let md = &ctx.metadata;
        let v1 = md.get("key-1").unwrap();
        let v2 = md.get("key-2").unwrap();

        assert_eq!(v1[0], "value-1-1");
        assert_eq!(v1[1], "value-1-2");
        assert_eq!(v2[0], "value-2");
        assert_eq!(&req.id, "id1");

        Ok(CreateTaskResponse { pid: 0x10c0 })
    }
}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    let server = async move {
        Server::new()
            .register(service!(FakeServer::new() : Task))
            .bind("unix:///tmp/shim-proto-ttrpc-001")
            .await
            .unwrap();
    };

    let ctrl_c = async move {
        ctrl_c().await.expect("Failed to wait for Ctrl+C.");
    };

    tokio::select! {
        _ = server => {},
        _ = ctrl_c => {},
    };
}
