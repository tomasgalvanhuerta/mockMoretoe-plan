use super::plan::Plan;

#[derive(Debug, Clone, Default)]
pub struct CreatePlanDomain {
    // pub create_state: CreateState,
    pub plan: Plan,
    pub index: i32,
    pub name: String,
    pub subtitle: String,
}

#[derive(Debug, Clone, Copy)]
pub enum CreateState {
    Name,
    Subtitle,
}
