use std::path::{Path, PathBuf};

use derive_more::Deref;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Deref)]
#[serde(transparent)]
pub struct FolderPath(pub PathBuf);

#[rustfmt::skip]
impl FolderPath {
    pub fn contains(&self, rel: &Path) -> bool { rel.starts_with(self.as_path()) }
}

#[cfg(test)]
mod tests {
    use super::FolderPath;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let folders = [FolderPath(PathBuf::from("src/core"))];
        assert!(folders[0].contains(Path::new("src/core/x.rs")));
        assert!(!folders[0].contains(Path::new("src/other/x.rs")));
    }
}
