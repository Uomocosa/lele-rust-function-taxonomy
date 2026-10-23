use std::path::Path;
use std::process::Command;

use crate::toolchain::Toolchain;

pub fn apply_pinned_env(command: &mut Command) {
    let bin_dir = Path::new(&Toolchain::sysroot()).join("bin");
    let path = {
        let existing = std::env::var("PATH").unwrap_or_default();
        format!("{}:{existing}", bin_dir.display())
    };
    command
        .env("RUSTC", bin_dir.join("rustc"))
        .env("PATH", path);
}

#[cfg(test)]
mod tests {
    use super::apply_pinned_env;
    use std::process::Command;

    #[test]
    fn test_usage() {
        let mut command = Command::new("cargo");
        apply_pinned_env(&mut command);
        assert!(command.get_envs().any(|(key, _)| key == "RUSTC"));
    }
}
