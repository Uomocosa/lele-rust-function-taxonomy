use std::path::{Path, PathBuf};

use crate::toolchain;

pub fn pinned_cargo() -> PathBuf {
    let candidate = Path::new(&toolchain::sysroot()).join("bin/cargo");
    if candidate.is_file() {
        candidate
    } else {
        PathBuf::from(toolchain::cargo())
    }
}

// no test_usage necessary
