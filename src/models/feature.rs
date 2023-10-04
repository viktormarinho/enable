pub use super::*;

#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub project_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentFeature {
    pub id: String,
    pub active: bool,
    pub feature_id: String,
    pub environment_id: String
}