use std::io::Write;
use std::collections::HashMap;

use crate::store::Store;
use crate::task::{Task, TaskType};

type Result = std::result::Result<(), std::io::Error>;

pub fn render<T: Write>(buf: &mut T, store: &Store) -> Result {
    Renderer::new(buf, store).render()
}

const COLORS: &'static [&'static str] = &[
    "orange",
    "gold",
    "coral",
    "darkgreen",
    "pink"
];

struct Config {
    type_color_map: HashMap<TaskType, &'static str>
}

impl Config {
    fn new(store: &Store) -> Self {
        let mut type_color_map = HashMap::new();
        for (i, tp) in store.task_types.iter().enumerate() {
            if COLORS.len() > i {
                type_color_map.insert(tp.clone(), COLORS[i]);
            }
        }
        Self { type_color_map }
    }
}

struct Renderer<'a, T> {
    buf: T,
    store: &'a Store,
    config: Config
}

impl<'a, T: Write> Renderer<'a, T> {
    fn new(buf: T, store: &'a Store) -> Self {
        let config = Config::new(store);
        Self { buf, store, config }
    }

    fn render(&mut self) -> Result {
        write!(self.buf, "digraph G {{\n")?;
        self.define_nodes()?;
        self.define_deps()?;
        self.define_legend()?;
        write!(self.buf, "}}\n")
    }

    fn define_nodes(&mut self) -> Result {
        write!(self.buf, "  # Declare nodes\n")?;
        for task in self.store.tasks.values() {
            self.define_node(task)?;
        }
        write!(self.buf, "\n")
    }

    fn define_node(&mut self, task: &Task) -> Result {
        let label = build_label(task);

        write!(self.buf, "  \"{}\" ", task.id)?;
        write!(self.buf, "[")?;
        write!(self.buf, "label=\"{}\"", label)?;
        if let Some(tp) = &task.task_type {
            if let Some(color) = self.config.type_color_map.get(tp) {
                write!(self.buf, " style=filled color={}", color)?;
            }
        }
        write!(self.buf, "]\n")
    }

    fn define_deps(&mut self) -> Result {
        write!(self.buf, "  # Declare dependencies\n")?;
        for task in self.store.tasks.values() {
            for dep in task.deps.iter() {
                write!(self.buf, "  \"{}\" -> \"{}\"\n", dep, task.id)?;
            }
        }

        Ok(())
    }

    fn define_legend(&mut self) -> Result {
        write!(self.buf, "\n  # Define Legend\n")?;
        write!(self.buf, "  subgraph cluster_1 {{\n")?;
        write!(self.buf, "    rank = sink\n")?;
        write!(self.buf, "    label = \"Legend\"\n")?;

        for (tp, color) in self.config.type_color_map.iter() {
            write!(self.buf, "    \"{}\" [style=filled color={}]\n", tp, color)?;
        }
        write!(self.buf, "  }}\n")
    }
}

fn build_label(task: &Task) -> String {
    let mut label = format!("{}", task.id);
    if let Some(name) = &task.name {
        label.push_str(&format!("\\n{}", name));
    }
    label
}
