use derive_more::{Display, Constructor};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Display, Constructor)]
pub struct TaskId(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Display, Constructor)]
pub struct TaskType(String);

#[derive(Debug, PartialEq, Eq, Display, Constructor)]
pub struct TaskName(String);

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub name: Option<TaskName>,
    pub deps: Vec<TaskId>,
    pub task_type: Option<TaskType>
}

impl<T: Into<String>> From<T> for TaskId  {
    fn from(id: T) -> Self {
        TaskId(id.into())
    }
}
