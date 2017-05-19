use model::task::Task;
use model::datetime::DateTime;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u32,
    pub comment: String,
    pub task: Task,
    pub start: DateTime,
    pub end: Option<DateTime>
}
