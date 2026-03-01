use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct RepsDecode {
    pub index: i32,
    pub completedDate: i64,
    pub percentage: i32,
    pub repeating: i32,
    pub date: i64,
    pub widgetId: Uuid,
    pub completed: bool,
    pub id: Uuid,
    pub amount: i32,
}
