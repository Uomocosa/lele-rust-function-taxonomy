use std::path::Path;

use crate::lele_config;

pub fn is_inside_boundary(rel: &Path, folders: &[lele_config::FolderPath]) -> bool {
    folders
        .iter()
        .any(|folder| rel.starts_with(folder.as_path()))
}

#[cfg(test)]
mod tests {
    use super::is_inside_boundary;
    use crate::lele_config;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let folders = [lele_config::FolderPath(PathBuf::from("src/core"))];
        assert!(is_inside_boundary(Path::new("src/core/x.rs"), &folders));
        assert!(!is_inside_boundary(Path::new("src/other/x.rs"), &folders));
    }
}
