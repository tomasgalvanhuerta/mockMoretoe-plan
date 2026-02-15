use chrono::NaiveDate;

#[derive(Default)]
struct Plan {
    author: String,
    guid: String,
    index: i32,
    inProgress: bool,
    name: String,
    started: NaiveDate,
    subtitle: String,
}
