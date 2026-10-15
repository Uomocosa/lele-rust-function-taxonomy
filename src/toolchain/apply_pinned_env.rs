use std::path::Path;

use crate::toolchain;

pub fn apply_pinned_env(command: &mut std::process::Command) {
    let bin_dir = Path::new(&toolchain::sysroot()).join("bin");
    let path = {
        let existing = std::env::var("PATH").unwrap_or_default();
        format!("{}:{existing}", bin_dir.display())
    };
    command
        .env("RUSTC", bin_dir.join("rustc"))
        .env("PATH", path);
}

// no test_usage necessary
