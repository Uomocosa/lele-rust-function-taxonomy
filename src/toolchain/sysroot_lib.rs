use std::path::{Path, PathBuf};

use crate::toolchain;

pub fn sysroot_lib() -> PathBuf {
    Path::new(&toolchain::sysroot()).join("lib")
}

// no test_usage necessary
