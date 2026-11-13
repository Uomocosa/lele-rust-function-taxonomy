use std::path::PathBuf;
use std::process::Command;

use crate::basic::enums::Outcome;
use crate::constants;
use crate::lele_config;
use crate::toolchain::Toolchain;
use crate::FailureReason;

pub fn run(manifest_path: Option<PathBuf>) -> Outcome {
    let manifest = manifest_path.unwrap_or_else(|| PathBuf::from("./Cargo.toml"));
    let Ok(manifest) = std::fs::canonicalize(&manifest) else {
        return Outcome::Failed(FailureReason::ManifestNotFound { path: manifest });
    };

    let config = match lele_config::Config::try_from(manifest.as_path()) {
        Ok(config) => config,
        Err(e) => return Outcome::Failed(FailureReason::Config(e)),
    };

    if !config.has_honest_boundary() {
        return Outcome::NothingToCheck;
    }

    if let Err(e) = config.validate(manifest.as_path()) {
        return Outcome::Failed(FailureReason::Config(e));
    }

    let Some(driver) = Toolchain::driver_path().or_else(Toolchain::build_driver) else {
        return Outcome::Failed(FailureReason::DriverUnavailable);
    };

    let target_dir = Toolchain::target_dir();

    let lib_path = {
        let existing = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
        let sysroot_lib = Toolchain::sysroot_lib();
        if existing.is_empty() {
            sysroot_lib.to_string_lossy().into_owned()
        } else {
            format!("{}:{}", sysroot_lib.display(), existing)
        }
    };

    let mut command = Command::new(Toolchain::pinned_cargo());
    command
        .arg("check")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--lib")
        .arg("--bins")
        .env("RUSTC_WORKSPACE_WRAPPER", &driver)
        .env(constants::ENV_MANIFEST, &manifest)
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("LD_LIBRARY_PATH", lib_path)
        .env("CARGO_BUILD_JOBS", constants::BUILD_JOBS);
    Toolchain::apply_pinned_env(&mut command);

    match command.status() {
        Ok(status) if status.success() => Outcome::Clean,
        Ok(_) => Outcome::Failed(FailureReason::CargoCheckFailed),
        Err(error) => Outcome::Failed(FailureReason::CargoSpawn { error }),
    }
}

// no test_usage necessary
