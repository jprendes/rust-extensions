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

use containerd_shim_protos::api::CreateTaskRequest;
use containerd_shim_protos::Task as _;
use trapeze::Client;

#[tokio::main]
async fn main() {
    let c = Client::connect("unix:///tmp/shim-proto-ttrpc-001")
        .await
        .unwrap();
    let c = c.with_metadata([
        ("key-1", "value-1-1"),
        ("key-1", "value-1-2"),
        ("key-2", "value-2"),
    ]);

    let now = std::time::Instant::now();

    let req = CreateTaskRequest {
        id: "id1".into(),
        ..Default::default()
    };
    println!(
        "OS Thread {:?} - task.create() started: {:?}",
        std::thread::current().id(),
        now.elapsed(),
    );
    let resp = c.create(req).await.unwrap();
    assert_eq!(resp.pid, 0x10c0);
    println!(
        "OS Thread {:?} - task.create() -> {:?} ended: {:?}",
        std::thread::current().id(),
        resp,
        now.elapsed(),
    );
}
