use std::collections::HashMap;

use thiserror::Error;

use crate::task::{TaskId, TaskType, Task};

#[derive(Debug)]
pub struct Store {
    pub tasks: HashMap<TaskId, Task>,
    pub task_types: Vec<TaskType>
}

impl Store {
    pub fn builder() -> StoreBuilder {
        StoreBuilder::new()
    }

    pub fn get(&self, id: &TaskId) -> &Task {
        // Unwrap is safe, because all ids were validated by StoreBuilder
        self.tasks.get(id).unwrap()
    }
}

#[derive(Debug, Error)]
pub enum StoreBuilderError {
    #[error("Task '{host}' refers to '{dep}', but '{dep}' is not defined.")]
    MissingDependecy {
        host: TaskId,
        dep: TaskId
    }
}

#[derive(Debug)]
pub struct StoreBuilder {
    tasks: Vec<Task>,
    task_types: Vec<TaskType>
}

impl StoreBuilder {
    fn new() -> Self {
        Self {
            tasks: vec![] ,
            task_types: vec![],
        }
    }

    pub fn add(mut self, task: Task) -> Self {
        if let Some(tp) = &task.task_type {
            if !self.task_types.contains(tp) {
                self.task_types.push((*tp).clone())
            }
        }
        self.tasks.push(task);
        self
    }

    pub fn build(self) -> Result<Store, StoreBuilderError> {
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

        let store = Store { tasks: tasks_map, task_types: self.task_types };
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
