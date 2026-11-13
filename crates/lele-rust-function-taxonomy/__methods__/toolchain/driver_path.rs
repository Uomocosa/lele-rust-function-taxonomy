use std::path::PathBuf;

use crate::basic::constants;
use crate::toolchain::Toolchain;

pub fn driver_path() -> Option<PathBuf> {
    if let Ok(explicit) = std::env::var(constants::ENV_DRIVER) {
        let candidate = PathBuf::from(explicit);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let built = Toolchain::driver_target_dir().join("debug/lele-taxonomy-driver");
    if built.is_file() {
        return Some(built);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::driver_path;

    #[test]
    fn test_usage() {
        assert!(driver_path().is_none_or(|path| path.is_file()));
    }
}
