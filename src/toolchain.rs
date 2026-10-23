use std::path::{Path, PathBuf};
use std::process::Command;

use atomic_delegate_macros::atomic_delegates;

use crate::basic::constants;

pub struct Toolchain;

impl Toolchain {
    pub fn cargo() -> String {
        std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
    }

    pub fn sysroot() -> String {
        option_env!("LELE_TAXONOMY_SYSROOT")
            .map(str::to_string)
            .unwrap_or_default()
    }

    pub fn sysroot_lib() -> PathBuf {
        Path::new(&Self::sysroot()).join("lib")
    }

    pub fn pinned_cargo() -> PathBuf {
        let candidate = Path::new(&Self::sysroot()).join("bin/cargo");
        if candidate.is_file() {
            candidate
        } else {
            PathBuf::from(Self::cargo())
        }
    }

    pub fn home_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home)
    }

    pub fn manifest_dir() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(manifest_dir)
    }

    pub fn target_dir() -> PathBuf {
        if let Ok(existing) = std::env::var(constants::ENV_TARGET_DIR) {
            if !existing.is_empty() {
                return PathBuf::from(existing);
            }
        }
        Self::home_dir().join(constants::CACHE_TARGET_DIR)
    }

    pub fn driver_target_dir() -> PathBuf {
        Self::home_dir().join(constants::DRIVER_CACHE_TARGET_DIR)
    }
}

#[atomic_delegates]
impl Toolchain {
    pub fn driver_path() -> Option<PathBuf> {}
    pub fn build_driver() -> Option<PathBuf> {}
    pub fn apply_pinned_env(command: &mut Command) {}
}

#[cfg(test)]
mod tests {
    use crate::toolchain::Toolchain;

    #[test]
    fn test_usage() {
        assert_ne!(Toolchain::cargo(), "");
        assert!(Toolchain::sysroot_lib().ends_with("lib"));
        assert!(Toolchain::target_dir()
            .to_string_lossy()
            .contains("lele_taxonomy"));
    }
}
