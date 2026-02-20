use super::reps_decode::RepsDecode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LiftingDecode {
    reps: Vec<RepsDecode>,
    medium: String, // Can be a fixed string

    #[serde(rename = "bodyPart")]
    body_part: String, // Can be Fixed String

    #[serde(rename = "type")]
    workout_type: String,

    #[serde(rename = "liftingType")]
    lifting_type: String,
    author: String,
    name: String,
    #[serde(rename = "standardTimer")]
    standard_timer: i32,
    guid: Uuid,
}
