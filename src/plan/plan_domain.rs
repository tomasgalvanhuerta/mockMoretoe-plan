use super::plan::Plan;
use super::plan_cell_domain::PlanCellDomain;
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
    Delete(i32),
    UserInteraction(UserInteraction),
}

#[derive(Debug, Clone)]
enum PlanSubscription {
    StartSubsciptions,
    SubscribeToInsertedPlans,
    SubscribeToDeletedPlans,
    SubscribeToTrainingCycleAdded,

    HandleInsertedPlan(Box<Plan>),
    HandleDeletedPlan,
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
