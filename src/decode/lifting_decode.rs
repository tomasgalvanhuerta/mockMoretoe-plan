use super::reps_decode::RepsDecode;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

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
