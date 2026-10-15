use std::path::PathBuf;

use crate::constants;
use crate::toolchain;

pub fn build_driver() -> Option<PathBuf> {
    let target_dir = toolchain::driver_target_dir();
    let mut command = std::process::Command::new(toolchain::pinned_cargo());
    command
        .arg("build")
        .arg("--manifest-path")
        .arg(toolchain::manifest_dir().join("Cargo.toml"))
        .arg("--bin")
        .arg("lele-taxonomy-driver")
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("CARGO_BUILD_JOBS", constants::BUILD_JOBS);
    toolchain::apply_pinned_env(&mut command);
    let status = command.status().ok()?;
    if !status.success() {
        return None;
    }
    let candidate = target_dir.join("debug/lele-taxonomy-driver");
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}

// no test_usage necessary
