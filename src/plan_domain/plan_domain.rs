use iced::Subscription;

use super::plan_cell_domain::PlanCellDomain;
use crate::plan::plan::Plan;
use std::boxed::Box;

#[derive(Default)]
struct PlanState {
    plan: Vec<Plan>,
    cells: Vec<PlanCellDomain>,
    // trainingCycleStruct
    // folders
    // dbReference
}

#[derive(Debug, Clone)]
enum PlanAction {
    Initialize,
    Subscriptions(PlanSubscription),
    FetchLocalPlan,
    HandleLocalPlans(i32),
    ReInitializeState,
    HandleNewPlan(i32),
    HandlePresentPlan,
    MergePlansToCycle,
    Delete(Box<String>),
    UserInteraction(UserInteraction),
}

#[derive(Debug, Clone)]
enum PlanSubscription {
    StartSubsciptions,
    SubscribeToInsertedPlans,
    SubscribeToDeletedPlans,
    SubscribeToTrainingCycleAdded,

    HandleInsertedPlan(Box<Plan>),
    HandleDeletedPlan(Box<String>),
}

#[derive(Debug, Clone)]
enum UserInteraction {
    TappedPresentPlan,
    TappedPlan,
    RefreshLis,
    TappedCreateTrainingCycle,
    CreateFolderWithPlans,
    PresentNewPlan,
}

impl PlanState {
    fn update(&self, action: PlanAction) {
        match action {
            PlanAction::Initialize => {}
            PlanAction::Subscriptions(subscription) => self.subscribe(subscription),
            PlanAction::Delete(id) => {
                println!("Delete plan using ID {:}", id)
            }
            PlanAction::FetchLocalPlan => {
                println!("Calling action {:?}", action)
            }
            PlanAction::HandleLocalPlans(i32) => {
                println!("Calling action {:?}", action)
            }
            PlanAction::ReInitializeState => {
                println!("Calling action {:?}", action)
            }
            PlanAction::HandleNewPlan(i32) => {
                println!("Calling action {:?}", action)
            }
            PlanAction::HandlePresentPlan => {
                println!("Calling action {:?}", action)
            }
            PlanAction::MergePlansToCycle => {
                println!("Calling action {:?}", action)
            }
            PlanAction::Delete(id_of_plan) => {
                println!("Calling action {:?}", id_of_plan)
            }
            PlanAction::UserInteraction(user_interaction) => {
                self.user_interaction(user_interaction)
            }
        }
    }

    fn user_interaction(&self, action: UserInteraction) {
        match action {
            UserInteraction::TappedPresentPlan => {}
            UserInteraction::TappedPlan => {}
            UserInteraction::RefreshLis => {}
            UserInteraction::TappedCreateTrainingCycle => {}
            UserInteraction::CreateFolderWithPlans => {}
            UserInteraction::PresentNewPlan => {}
        }
    }

    fn subscribe(&self, action: PlanSubscription) {
        match action {
            PlanSubscription::StartSubsciptions => {
                self.update(PlanAction::Subscriptions(
                    PlanSubscription::SubscribeToInsertedPlans,
                ));
                self.update(PlanAction::Subscriptions(
                    PlanSubscription::SubscribeToInsertedPlans,
                ));
                self.update(PlanAction::Subscriptions(
                    PlanSubscription::SubscribeToDeletedPlans,
                ));
                self.update(PlanAction::Subscriptions(
                    PlanSubscription::SubscribeToTrainingCycleAdded,
                ));
            }
            PlanSubscription::SubscribeToInsertedPlans => {
                println!("Subscribe to SubscribeToInsertedPlans")
            }
            PlanSubscription::SubscribeToDeletedPlans => {
                println!("Subscribe to SubscribeToDeletedPlans")
            }
            PlanSubscription::SubscribeToTrainingCycleAdded => {
                println!("Subscribe to SubscribeToTrainingCycleAdded")
            }
            PlanSubscription::HandleDeletedPlan(idOfPlan) => {
                self.update(PlanAction::Delete(idOfPlan));
            }
            PlanSubscription::HandleInsertedPlan(plan) => {
                println!("Handle deleting Plan {:?}", plan)
            }
        }
    }
}
