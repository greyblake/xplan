use std::collections::HashMap;
use std::hash::Hash;

use crate::task::{TaskId, Task};

#[derive(Debug)]
pub struct Store {
    tasks: HashMap<TaskId, Task>
}

impl Store {
    fn builder() -> StoreBuilder {
        StoreBuilder::new()
    }

    fn get(&self, id: &TaskId) -> &Task {
        // Unwrap is safe, because all ids were validated by StoreBuilder
        self.tasks.get(id).unwrap()
    }
}

#[derive(Debug)]
pub enum StoreBuilderError {
    MissingDependecy {
        host: TaskId,
        dep: TaskId
    }
}

impl std::fmt::Display for StoreBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDependecy { host, dep } => {
                write!(f, "Task '{}' refers to '{}', but '{}' is not defined.", host, dep, dep)
            }
        }
    }
}

#[derive(Debug)]
struct StoreBuilder {
    tasks: Vec<Task>
}

impl StoreBuilder {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    fn build(self) -> Result<Store, StoreBuilderError> {
        let mut tasks_map: HashMap<TaskId, Task> = HashMap::new();

        let known_ids: Vec<TaskId> = self.tasks.iter().map(|u| u.id.clone()).collect();

        for task in self.tasks.into_iter() {
            // Check dependencies
            for dep_id in task.deps.iter() {
                if !known_ids.contains(dep_id) {
                    let e = StoreBuilderError::MissingDependecy { host: task.id.clone(), dep: dep_id.clone() };
                    return Err(e);
                }
            }

            // Add to HashMap
            tasks_map.insert(task.id.clone(), task);
        }

        let store = Store { tasks: tasks_map };
        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task<S: Into<TaskId>>(id: S) -> Task {
        task_with_deps(id, Vec::new())
    }

    fn task_with_deps<T: Into<TaskId>>(id: T, deps: Vec<T>) -> Task {
        Task {
            id: id.into(),
            name: None,
            deps: deps.into_iter().map(|d| d.into()).collect(),
            task_type: None
        }
    }

    #[test]
    fn test_builder_ok() {
        let store = Store::builder()
            .add(task("A"))
            .add(task("B"))
            .build()
            .unwrap();

        let a_id = TaskId::from("A");
        assert_eq!(store.get(&a_id), &task("A"));

        let b_id = TaskId::from("B");
        assert_eq!(store.get(&b_id), &task("B"));
    }

    #[test]
    fn test_builder_err() {
        let err = Store::builder()
            .add(task("A"))
            .add(task_with_deps("B", vec!["Z"]))
            .build()
            .unwrap_err();

        assert_eq!(
            format!("{}", err),
            "Task 'B' refers to 'Z', but 'Z' is not defined."
        )
    }
}
