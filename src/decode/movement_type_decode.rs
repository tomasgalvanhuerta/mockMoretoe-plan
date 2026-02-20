use super::lifting_decode::LiftingDecode;
use uuid::Uuid;

pub struct MovementTypeDecode {
    pub name: String,
    pub refId: Uuid,
    pub lifting: LiftingDecode,
}
