use crate::{Expression, Motion};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct FileReferences {
  pub display_info: PathBuf,
  #[serde(default)]
  pub expressions: Vec<Expression>,
  pub moc: PathBuf,
  pub motion_sync: Option<PathBuf>,
  pub motions: BTreeMap<String, Vec<Motion>>,
  pub physics: Option<PathBuf>,
  pub pose: Option<PathBuf>,
  pub textures: Vec<PathBuf>,
}
