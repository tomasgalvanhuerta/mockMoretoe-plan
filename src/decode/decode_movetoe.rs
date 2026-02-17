use uuid::Uuid;

pub struct MovementDecode {
    pub author: String,
    pub explanation: String,
    pub guid: Uuid,
    pub index: i32,
    pub isAuthor: bool,
    pub name: String,
    pub standardTimer: i32,

    pub image_ids: Vec<Uuid>,
}

struct GifImageDecode {
    pub id: Uuid,
    pub image_ids: Vec<Uuid>,
}

pub struct LiftingDecode {
    reps: Vec<RepsDecode>,
    medium: String,    // Can be a fixed string
    body_part: String, // Can be Fixed String
    lifting_type: String,
}

pub struct RepsDecode {}
