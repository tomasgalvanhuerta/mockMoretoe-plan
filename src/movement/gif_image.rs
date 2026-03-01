use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GifImageDecode {
    #[serde(rename = "refMovementGUID")]
    pub guid: Uuid,

    #[serde(rename = "gif")]
    pub image_ids: Vec<ImageToe>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageToe {
    #[serde(rename = "data")]
    pub data: String,
    pub index: i64,
}
