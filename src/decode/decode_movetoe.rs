use super::lifting_decode::LiftingDecode;
use super::movement_type_decode::MovementTypeDecode;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

pub struct MovementDecode {
    pub author: String,
    pub explanation: String,
    pub guid: Uuid,
    pub index: i32,
    pub isAuthor: bool,
    pub name: String,
    pub standardTimer: i32,
    pub superSet: Vec<Self>,

    pub image_ids: Vec<Uuid>,
}

struct GifImageDecode {
    pub id: Uuid,
    pub image_ids: Vec<i8>,
}
