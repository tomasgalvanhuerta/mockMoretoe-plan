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

pub struct MovementTypeDecode {
    pub name: String,
    pub refId: Uuid,
    pub lifting: LiftingDecode,
}

struct GifImageDecode {
    pub id: Uuid,
    pub image_ids: Vec<i8>,
}

pub struct LiftingDecode {
    reps: Vec<RepsDecode>,
    medium: String,    // Can be a fixed string
    body_part: String, // Can be Fixed String
    lifting_type: String,
    // type: String
    author: String,
    name: String,
    standardTimer: i32,
    guid: Uuid,
}

pub struct RepsDecode {}
