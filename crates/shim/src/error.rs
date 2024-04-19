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

use thiserror::Error;

use crate::{
    monitor::ExitEvent,
    protos::{prost, trapeze},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// Invalid command line arguments.
    #[error("Failed to parse command line: {0}")]
    InvalidArgument(String),

    /// TTRPC specific error.
    #[error("TTRPC error: {0}")]
    Ttrpc(#[from] trapeze::Status),

    #[error("Protobuf error: {0}")]
    Protobuf(#[from] prost::DecodeError),

    #[error("{context} error: {err}")]
    IoError {
        context: String,
        #[source]
        err: std::io::Error,
    },

    #[error("Env error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Failed to setup logger: {0}")]
    Setup(#[from] log::SetLoggerError),

    /// Unable to pass fd to child process (we rely on `command_fds` crate for this).
    #[cfg(unix)]
    #[error("Failed to pass socket fd to child: {0}")]
    FdMap(#[from] command_fds::FdMappingCollision),

    #[cfg(unix)]
    #[error("Nix error: {0}")]
    Nix(#[from] nix::Error),

    #[error("Failed to get envelope timestamp: {0}")]
    Timestamp(#[from] std::time::SystemTimeError),

    #[error("Not Found: {0}")]
    NotFoundError(String),

    #[error("Failed pre condition: {0}")]
    FailedPreconditionError(String),

    #[cfg(unix)]
    #[error("{context} error: {err}")]
    MountError {
        context: String,
        #[source]
        err: nix::Error,
    },

    #[error("Failed to convert json object: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Failed to send exit event: {0}")]
    Send(#[from] std::sync::mpsc::SendError<ExitEvent>),

    #[error("Other: {0}")]
    Other(String),

    #[error("Unimplemented method: {0}")]
    Unimplemented(String),
}

impl From<Error> for trapeze::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidArgument(message) => trapeze::Status {
                code: trapeze::Code::InvalidArgument as i32,
                message,
                details: vec![],
            },
            Error::NotFoundError(message) => trapeze::Status {
                code: trapeze::Code::NotFound as i32,
                message,
                details: vec![],
            },
            Error::FailedPreconditionError(message) => trapeze::Status {
                code: trapeze::Code::FailedPrecondition as i32,
                message,
                details: vec![],
            },
            Error::Ttrpc(status) => status,
            message => trapeze::Status {
                code: trapeze::Code::Unknown as i32,
                message: message.to_string(),
                details: vec![],
            },
        }
    }
}

#[macro_export]
macro_rules! io_error {
    ($e:ident, $($args:tt)+) => {
        |$e| $crate::error::Error::IoError {
            context: format_args!($($args)+).to_string(),
            err: $e,
        }
    };
}

#[macro_export]
macro_rules! mount_error {
    ($e:ident, $($args:tt)+) => {
        |$e| Error::MountError {
            context: format_args!($($args)+).to_string(),
            err: $e,
        }
    };
}

#[macro_export]
macro_rules! other {
    ($($args:tt)*) => {
        Error::Other(format_args!($($args)*).to_string())
    };
}

#[macro_export]
macro_rules! other_error {
    ($e:ident, $s:expr) => {
        |$e| Error::Other($s.to_string() + &": ".to_string() + &$e.to_string())
    };
}
