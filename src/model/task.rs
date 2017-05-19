use model::datetime::DateTime;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub created: DateTime,
    pub last_used: DateTime
}
