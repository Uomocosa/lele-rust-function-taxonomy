use std::path::PathBuf;

use crate::basic::constants;
use crate::toolchain;

pub fn driver_target_dir() -> PathBuf {
    toolchain::home_dir().join(constants::DRIVER_CACHE_TARGET_DIR)
}

// no test_usage necessary
