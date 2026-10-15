use std::path::PathBuf;

use crate::basic::constants;
use crate::toolchain;

pub fn target_dir() -> PathBuf {
    if let Ok(existing) = std::env::var(constants::ENV_TARGET_DIR) {
        if !existing.is_empty() {
            return PathBuf::from(existing);
        }
    }
    toolchain::home_dir().join(constants::CACHE_TARGET_DIR)
}

#[cfg(test)]
mod tests {
    use super::target_dir;

    #[test]
    fn test_usage() {
        assert!(target_dir().to_string_lossy().contains("lele_taxonomy"));
    }
}
