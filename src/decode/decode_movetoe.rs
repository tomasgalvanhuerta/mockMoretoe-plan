use super::gif_image::GifImageDecode;
use super::lifting_decode::LiftingDecode;
use super::movement_type_decode::MovementTypeDecode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct MovementDecode {
    pub author: String,
    pub explanation: String,
    pub guid: Uuid,
    pub index: i32,

    #[serde(rename = "isAuthor")]
    pub is_author: bool, // Using
    pub name: String,
    #[serde(rename = "standardTimer")]
    pub standard_timer: i32,
    #[serde(rename = "superSet")]
    pub super_set: Vec<Self>,

    pub image_ids: Vec<GifImageDecode>, // ID's to fetch images
}
