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

use trapeze_codegen::Config;

fn main() {
    Config::new()
        .enable_type_names()
        .include_file("mod.rs")
        .compile_protos(
            &[
                "vendor/gogoproto/gogo.proto",
                "vendor/github.com/containerd/containerd/protobuf/plugin/fieldpath.proto",
                "vendor/github.com/containerd/containerd/api/types/mount.proto",
                "vendor/github.com/containerd/containerd/api/types/task/task.proto",
                "vendor/github.com/containerd/cgroups/stats/v1/metrics.proto",
                "vendor/microsoft/hcsshim/cmd/containerd-shim-runhcs-v1/stats/stats.proto",
                "vendor/github.com/containerd/containerd/api/events/container.proto",
                "vendor/github.com/containerd/containerd/api/events/content.proto",
                "vendor/github.com/containerd/containerd/api/events/image.proto",
                "vendor/github.com/containerd/containerd/api/events/namespace.proto",
                "vendor/github.com/containerd/containerd/api/events/sandbox.proto",
                "vendor/github.com/containerd/containerd/api/events/snapshot.proto",
                "vendor/github.com/containerd/containerd/api/events/task.proto",
                "vendor/github.com/containerd/containerd/runtime/v2/runc/options/oci.proto",
                "vendor/github.com/containerd/containerd/api/runtime/task/v2/shim.proto",
                "vendor/github.com/containerd/containerd/api/services/ttrpc/events/v1/events.proto",
                #[cfg(feature = "sandbox")]
                "vendor/github.com/containerd/containerd/api/types/platform.proto",
                #[cfg(feature = "sandbox")]
                "vendor/github.com/containerd/containerd/api/runtime/sandbox/v1/sandbox.proto",
            ],
            &["vendor/"],
        )
        .expect("Failed to generate protos");
}

/*

*/
