use std::io::Write;

use crate::store::Store;

pub fn render<T: Write>(buf: &mut T, store: &Store) {
    write!(buf, "digraph G {{\n");

    declare_nodes(buf, store);
    declare_deps(buf, store);

    write!(buf, "}}\n");
}

pub fn declare_nodes<T: Write>(buf: &mut T, store: &Store) {
    write!(buf, "  # Declare nodes\n");
    for task in store.tasks.values() {
        write!(buf, "  \"{}\"\n", task.id);
    }
    write!(buf, "\n");
}

pub fn declare_deps<T: Write>(buf: &mut T, store: &Store) {
    write!(buf, "  # Declare dependencies\n");
    for task in store.tasks.values() {
        for dep in task.deps.iter() {
            write!(buf, "  \"{}\" -> \"{}\"\n", dep, task.id);
        }
    }
}
