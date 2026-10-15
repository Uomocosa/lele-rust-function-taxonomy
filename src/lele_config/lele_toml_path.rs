use std::path::{Path, PathBuf};

use crate::basic::constants;

pub fn lele_toml_path(manifest_path: &Path) -> PathBuf {
    manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(constants::LELE_TOML_FILE)
}

// no test_usage necessary
