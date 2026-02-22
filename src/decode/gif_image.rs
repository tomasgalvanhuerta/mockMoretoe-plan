use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct GifImageDecode {
    pub id: Uuid,
    pub image_ids: Vec<i8>,
}
