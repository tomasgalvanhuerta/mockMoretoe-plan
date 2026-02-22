use super::gif_image::GifImageDecode;
use super::lifting_decode::LiftingDecode;
use super::movement_type_decode::MovementTypeDecode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct MovementDecode {
    #[serde(default)]
    pub author: String,

    #[serde(default)]
    pub explanation: String,

    pub guid: Uuid,

    #[serde(default)]
    pub index: i32,

    #[serde(rename = "isAuthor")]
    #[serde(default)]
    pub is_author: bool, // Using

    #[serde(default)]
    pub name: String,

    #[serde(rename = "standardTimer")]
    pub standard_timer: i32,

    #[serde(rename = "superSet")]
    pub super_set: Vec<Self>,

    #[serde(rename = "images")]
    pub image_ids: GifImageDecode, // ID's to fetch images

    #[serde(rename = "movementType")]
    pub movement_type: MovementTypeDecode,
}
