use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
