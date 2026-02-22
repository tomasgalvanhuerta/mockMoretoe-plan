use super::lifting_decode::LiftingDecode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct MovementTypeDecode {
    pub name: String,

    #[serde(rename = "refId")]
    pub ref_id: Uuid,

    // This will eventually be an enum with different Movement Types
    pub lifting: LiftingDecode,
}
