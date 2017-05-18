use data::task::Task;
use data::datetime::DateTime;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub task: Task,
    pub start: DateTime,
    pub end: Option<DateTime>
}
