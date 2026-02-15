#[derive(Default)]
struct Plan {
    // trainingCycleStruct
    // folders
    // dbReference
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum PlanSubscription {
    Start,
}

#[derive(Debug, Clone, Copy)]
enum UserInteraction {
    TappedPresentPlan,
    TappedPlan,
    RefreshLis,
    TappedCreateTrainingCycle,
    CreateFolderWithPlans,
    PresentNewPlan,
}
