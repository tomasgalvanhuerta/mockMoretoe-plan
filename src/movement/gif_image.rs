use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct GifImageDecode {
    #[serde(rename = "refMovementGUID")]
    pub guid: String,

    #[serde(rename = "gif")]
    pub image_ids: Vec<ImageToe>,
}

#[derive(Debug, Deserialize)]
pub struct ImageToe {
    #[serde(rename = "data")]
    pub data: String,

    pub index: i64,
}
