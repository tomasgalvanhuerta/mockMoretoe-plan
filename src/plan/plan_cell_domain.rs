use super::plan::Plan;
use iced::widget::{Column, Image, Text, column, image, text};
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
    // userInteraction(UserInteraction), // shareButton
}

// #[derive(Debug, Clone, Copy)]
// enum UserInteraction {
//     tapped
// }

impl PlanCellDomain {
    fn update(&self, action: PlanCellAction) {
        match action {
            PlanCellAction::initialize => {
                println!("Fetch Image")
            }
            PlanCellAction::longGesture(flag) => {
                println!("Will open up sheet {flag}")
            }
        }
    }
}
