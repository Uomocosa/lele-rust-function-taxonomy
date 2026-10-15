use std::path::PathBuf;

use thiserror::Error;

use crate::lele_config;
use crate::Diagnostic;
use crate::Tax002;

#[derive(Debug, Error)]
pub enum FailureReason {
    #[error("{}: manifest not found: {}", Tax002.code(), path.display())]
    ManifestNotFound { path: PathBuf },
    #[error(transparent)]
    Config(#[from] lele_config::LeleTomlError),
    #[error("{}: could not locate or build the taxonomy driver", Tax002.code())]
    DriverUnavailable,
    #[error("{}: failed to run cargo check: {error}", Tax002.code())]
    CargoSpawn { error: std::io::Error },
    #[error("cargo check reported errors")]
    CargoCheckFailed,
}

// no test_usage necessary
