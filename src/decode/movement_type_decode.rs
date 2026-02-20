use super::lifting_decode::LiftingDecode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct MovementTypeDecode {
    pub name: String,

    #[serde(rename = "refId")]
    pub ref_id: Uuid,
    pub lifting: LiftingDecode,
}
