use super::plan::Plan;
use chrono::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct StoryShareButton {
    pub date: NaiveDate,
    pub title: String,
    pub movement_types: Vec<String>, // Movement Types
    pub transferable_plan: Plan,
    pub transferable_with_images: Plan,
    pub create_plan_state: String, // Create Plan Domain
}

pub enum StoryShareAction {
    initialize,
    delete,
    create_plan,
    addplan(Box<Plan>),
}

impl StoryShareButton {
    fn update(&self, action: StoryShareAction) {}
}
