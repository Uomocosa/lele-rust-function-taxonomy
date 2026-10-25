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
    #[error("{}: {file}: {}", Tax002.code(), Self::first_line(.toml_error))]
    InvalidToml {
        file: lele_config::ConfigSource,
        toml_error: toml::de::Error,
    },
}

impl LeleTomlError {
    pub(crate) fn first_line(error: &toml::de::Error) -> String {
        let message = error.to_string();
        message
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or(&message)
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::LeleTomlError;

    #[test]
    fn test_usage() {
        let err = toml::from_str::<toml::Value>("= invalid").unwrap_err();
        assert_ne!(LeleTomlError::first_line(&err), "");
    }
}
