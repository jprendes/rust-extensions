// Copyright (c) 2021 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use containerd_shim_protos::api::{CreateTaskRequest, CreateTaskResponse, DeleteRequest};
use containerd_shim_protos::Task;
use trapeze::{service, Client, Code, Result, ServerConnection};

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
        let resp = CreateTaskResponse { pid: 0x10c0 };
        assert_eq!(&req.id, "test1");
        Ok(resp)
    }
}

#[tokio::test]
async fn test_create_task() {
    let (client, server) = tokio::io::duplex(1024);
    tokio::spawn(async move {
        ServerConnection::new(server)
            .register(service!(FakeServer::new() : Task))
            .start()
            .await
    });
    let client = Client::new(client);

    let req = CreateTaskRequest {
        id: "test1".into(),
        ..Default::default()
    };
    let resp = client.create(req).await.unwrap();

    assert_eq!(resp.pid, 0x10c0);
}

#[tokio::test]
async fn test_delete_task() {
    let (client, server) = tokio::io::duplex(1024);
    tokio::spawn(async move {
        ServerConnection::new(server)
            .register(service!(FakeServer::new() : Task))
            .start()
            .await
    });
    let client = Client::new(client);

    let req = DeleteRequest {
        id: "test1".into(),
        ..Default::default()
    };
    let status = client.delete(req).await.unwrap_err();

    assert_ne!(status.code, Code::Ok as i32);
}
