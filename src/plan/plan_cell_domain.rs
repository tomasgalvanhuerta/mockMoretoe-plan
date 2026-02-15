use super::plan::Plan;
use std::boxed::Box;

#[derive(Default)]
pub struct PlanCellDomain {
    plan: Box<Plan>,
    layoutVertical: bool,
}

#[derive(Debug, Clone, Copy)]
enum PlanCellAction {
    initialize,
    longGesture(bool),
    userInteraction(UserInteraction), // shareButton
}

#[derive(Debug, Clone, Copy)]
enum UserInteraction {}
