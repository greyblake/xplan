pub mod error;
pub mod helpers;

use error::ParseError;
use helpers::{parse_yaml_to_string, parse_yaml_to_vec};
use crate::task::{Task, TaskId, TaskType, TaskName};
use crate::store::Store;

use yaml_rust::{yaml::Yaml, YamlLoader};

pub type Result<T> = std::result::Result<T, ParseError>;


fn parse_tasks(yaml: &Yaml) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];

    match yaml {
        Yaml::Hash(hash) => {
            for (yaml_key, val) in hash.iter() {
                let task = parse_task(yaml_key, val)?;
                tasks.push(task);
            }
        }
        _ => {
            let message = format!("`tasks` must be a hash. Got {:?}", yaml);
            return Err(ParseError::Base(message));
        }
    }

    Ok(tasks)
}

pub fn parse_task(key: &Yaml, body: &Yaml) -> Result<Task> {
    let id_str = parse_yaml_to_string(key)?;
    let id = TaskId::new(id_str);

    let mut name = None;
    let mut deps = vec![];
    let mut task_type = None;

    match body {
        Yaml::Hash(hash) => {
            for (attr_yaml_key, attr_yaml_val) in hash {
                let attr_key = parse_yaml_to_string(&attr_yaml_key)?;

                match attr_key.as_ref() {
                    "name" => {
                        let name_val = parse_yaml_to_string(&attr_yaml_val)?;
                        name = Some(TaskName::new(name_val));
                    },
                    "deps" => {
                        let deps_str = parse_yaml_to_vec(&attr_yaml_val)?;
                        deps = deps_str.into_iter().map(TaskId::from).collect();

                    },
                    "type" => {
                        let type_val = parse_yaml_to_string(&attr_yaml_val)?;
                        task_type = Some(TaskType::new(type_val));

                    }
                    _ => {
                        let msg = format!("Unknown task property `{}` in tasks.{}", attr_key, id);
                        return Err(ParseError::Base(msg))
                    }
                }
            }
        },
        Yaml::Null => {},
        _ => {
            let msg = format!("Invalid type of element: tasks.{} ({:?})", id, body);
            return Err(ParseError::Base(msg))
        }
    }

    let task = Task { id, name, deps, task_type };
    Ok(task)
}


pub fn parse(yaml: &str) -> Result<Store> {
    let docs = YamlLoader::load_from_str(yaml).map_err(ParseError::InvalidYaml)?;

    let mut tasks = vec![];

    for doc in docs.iter() {
        match doc {
            Yaml::Hash(root) => {
                for (yaml_key, val) in root.iter() {
                    let key = parse_yaml_to_string(yaml_key)?;

                    match key.as_ref() {
                        "tasks" => {
                            tasks = parse_tasks(val)?;
                        }
                        _ => {
                            return Err(ParseError::UnkownRootElement(key));
                        }
                    };
                }
            }
            _ => {
                return Err(ParseError::Base("Root element of YAML must be Hash".to_owned()))
            }
        }
    }

    let mut builder = Store::builder();
    for task in tasks.into_iter() {
        builder = builder.add(task);
    }

    let store = builder.build().map_err(ParseError::Build)?;


    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let yaml = r#"
            tasks:
              A:
                name: "Do A"
              B:
                deps: ["A"]
              C:
                type: BE
        "#;
        let store = parse(yaml).unwrap();
        assert_eq!(store.tasks.len(), 3);

        let id_a = TaskId::new("A".to_owned());
        assert_eq!(store.get(&id_a).name, Some(TaskName::new("Do A".to_owned())));

        let id_b = TaskId::new("B".to_owned());
        assert_eq!(store.get(&id_b).deps, vec![id_a]);

        let id_c = TaskId::new("C".to_owned());
        let type_be = TaskType::new("BE".to_owned());

        assert_eq!(store.get(&id_c).task_type, Some(type_be));
    }
}
