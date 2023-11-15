use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Model3 {
  pub file_references: FileReferences,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub groups: Vec<Group>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub hit_areas: Vec<HitArea>,
  pub version: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Expression {
  pub file: PathBuf,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Motion {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fade_in_time: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fade_out_time: Option<f64>,
  pub file: PathBuf,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub motion_sync: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sound: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Group {
  pub ids: Vec<String>,
  pub name: String,
  pub target: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct HitArea {
  pub id: String,
  pub name: String,
}
