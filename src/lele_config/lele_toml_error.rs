use thiserror::Error;

use crate::lele_config;
use crate::Diagnostic;
use crate::Tax002;

#[derive(Debug, Error)]
pub enum LeleTomlError {
    #[error("{}: {file}: {reason}", Tax002.code())]
    Invalid {
        file: lele_config::ConfigSource,
        reason: lele_config::ErrorReason,
    },
}
