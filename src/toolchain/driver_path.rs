use std::path::PathBuf;

use crate::basic::constants;
use crate::toolchain;

pub fn driver_path() -> Option<PathBuf> {
    if let Ok(explicit) = std::env::var(constants::ENV_DRIVER) {
        let candidate = PathBuf::from(explicit);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let built = toolchain::driver_target_dir().join("debug/lele-taxonomy-driver");
    if built.is_file() {
        return Some(built);
    }
    None
}

// no test_usage necessary
