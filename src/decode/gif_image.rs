use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct GifImageDecode {
    #[serde(rename = "refMovementGUID")]
    pub guid: Uuid,

    #[serde(rename = "data")]
    pub image_ids: Vec<u8>,
}
