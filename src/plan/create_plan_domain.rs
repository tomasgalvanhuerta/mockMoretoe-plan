use super::plan::Plan;

#[derive(Debug, Clone)]
pub struct CreatePlanDomain {
    pub create_state: CreateState,
    pub plan: Plan,
    pub index: i32,
    pub name: String,
    pub subtitle: String,
}

impl Default for CreatePlanDomain {
    fn default() -> Self {
        Self {
            create_state: CreateState::Name,
            plan: Plan::default(),
            index: i32::default(),
            name: String::default(),
            subtitle: String::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CreateState {
    Name,
    Subtitle,
}
