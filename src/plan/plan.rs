// use super::movement::Movement;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Plan {
    pub author: String,
    pub guid: Uuid,
    pub index: i32,
    pub in_progress: bool,
    pub name: String,
    pub started: NaiveDate,
    pub subtitle: String,
    pub movement_ids: Vec<Uuid>,

    pub image_ids: String, // Transferable Image
}

// impl Plan { // Should be in it's own module
//     fn movements(&self) -> Vec<Movement> {
//         Vec::new()
//     }
//     fn image_ids(&self) -> Vec<String> {
//         Vec::new()
//     }
// }
