use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct GifImageDecode {
    #[serde(rename = "refMovementGUID")]
    pub guid: Uuid,

    #[serde(rename = "gif")]
    pub image_ids: ImageToe,
}

#[derive(Debug, Deserialize)]
pub struct ImageToe {
    #[serde(rename = "data")]
    pub image: Vec<u8>,
}
