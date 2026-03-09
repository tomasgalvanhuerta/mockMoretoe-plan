use crate::movement::gif_image::GifImageDecode;
use crate::movement::movement::Movement;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Plan {
    pub author: String,
    pub guid: Uuid,
    pub index: i32,
    pub in_progress: bool,
    pub name: String,
    // pub started: NaiveDate,
    pub subtitle: String,

    #[serde(rename = "movement")]
    pub movements: Vec<Movement>,

    pub images: Vec<GifImageDecode>,
}
