use std::path::{Path, PathBuf};

use derive_more::{Deref, From};

use crate::basic::constants;
use crate::lele_config;

#[derive(Debug, Clone, PartialEq, Eq, Deref, From)]
#[from(forward)]
pub struct Manifest(pub PathBuf);

impl Manifest {
    pub fn lele_toml_path(&self) -> PathBuf {
        let parent = self.parent().unwrap_or_else(|| Path::new("."));
        parent.join(constants::LELE_TOML_FILE)
    }

    pub fn config_source(&self) -> lele_config::ConfigSource {
        let folder = self
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str());
        lele_config::ConfigSource(folder.map_or_else(
            || "lele.toml".to_string(),
            |name| format!("{name}/lele.toml"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Manifest;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let manifest = Manifest::from(Path::new("/tmp/crate/Cargo.toml"));
        assert!(manifest.lele_toml_path().ends_with("lele.toml"));
        assert_eq!(manifest.config_source().as_str(), "crate/lele.toml");
    }
}
