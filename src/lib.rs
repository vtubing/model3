mod expression;
mod file_references;
mod group;
mod hit_area;
mod motion;

pub use expression::Expression;
pub use file_references::FileReferences;
pub use group::Group;
pub use hit_area::HitArea;
pub use motion::Motion;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
