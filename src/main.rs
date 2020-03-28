use std::collections::HashMap;
use std::hash::Hash;

use xplan::task::{TaskId, Task};
use xplan::store::{Store};
use xplan::dot::render;


fn main() {
    let store = Store::builder()
        .add(task("A"))
        .add(task_with_deps("B", vec!["A"]))
        .add(task_with_deps("C", vec!["A", "B"]))
        .add(task_with_deps("D", vec!["A", "C"]))
        .build()
        .unwrap();

    let mut stdout = std::io::stdout();
    render(&mut stdout, &store);
}

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
