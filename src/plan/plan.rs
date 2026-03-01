use crate::movement::gif_image::GifImageDecode;
use crate::movement::movement::Movement;
use serde::Deserialize;
// use crate::plan::plan::Plan;
use chrono::NaiveDate;
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
    pub movement_ids: Vec<Movement>,

    pub images: GifImageDecode,
}
