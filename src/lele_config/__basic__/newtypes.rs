use std::path::PathBuf;

use derive_more::{Deref, Display};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Deref)]
#[serde(transparent)]
pub struct BoundaryName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Deref)]
#[serde(transparent)]
pub struct CallPath(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Deref)]
#[serde(transparent)]
pub struct FolderPath(pub PathBuf);

#[derive(Debug, Clone, PartialEq, Eq, Display, Deref)]
pub struct ConfigSource(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Display, Deref)]
pub struct ErrorReason(pub String);
