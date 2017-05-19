use model::datetime::DateTime;
use model::task::Task;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
    pub created: DateTime,
    pub last_used: DateTime
}
