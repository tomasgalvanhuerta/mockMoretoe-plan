use super::reps_decode::RepsDecode;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LiftingDecode {
    reps: Vec<RepsDecode>,
    #[serde(default)]
    medium: String, // Can be a fixed string

    #[serde(rename = "bodyPart")]
    body_part: String, // Can be Fixed String

    #[serde(rename = "type")]
    workout_type: String,

    #[serde(rename = "liftingType")]
    #[serde(default)]
    lifting_type: String,

    #[serde(default)]
    author: String,

    #[serde(default)]
    name: String,

    #[serde(rename = "standardTimer")]
    #[serde(default)]
    standard_timer: i32,

    #[serde(default)]
    guid: Uuid,
}
