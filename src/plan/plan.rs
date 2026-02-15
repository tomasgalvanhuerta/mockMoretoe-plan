use chrono::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct Plan {
    pub author: String,
    pub guid: String,
    pub index: i32,
    pub in_progress: bool,
    pub name: String,
    pub started: NaiveDate,
    pub subtitle: String,
}
