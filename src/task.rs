#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TaskId(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TaskType(String);

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub name: Option<String>,
    pub deps: Vec<TaskId>,
    pub task_type: Option<TaskType>
}

impl TaskId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl<T: Into<String>> From<T> for TaskId  {
    fn from(id: T) -> Self {
        TaskId(id.into())
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TaskType {
    pub fn new(val: String) -> Self {
        Self(val)
    }
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
