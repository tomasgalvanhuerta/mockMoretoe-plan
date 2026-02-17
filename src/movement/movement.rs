use uuid::Uuid;

#[derive(Default)]
pub struct Movement {
    pub author: String,
    pub explanation: String,
    pub guid: Uuid,
    pub index: i32,
    pub isAuthor: bool,
    pub name: String,
    pub standardTimer: i32,

    pub image_ids: Vec<Uuid>,
}
