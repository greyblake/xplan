use std::collections::HashMap;
use std::hash::Hash;

use crate::unit::{UnitId, Unit};

#[derive(Debug)]
pub struct Store {
    units: HashMap<UnitId, Unit>
}

impl Store {
    fn builder() -> StoreBuilder {
        StoreBuilder::new()
    }

    fn get(&self, id: &UnitId) -> &Unit {
        // Unwrap is safe, because all ids were validated by StoreBuilder
        self.units.get(id).unwrap()
    }
}

#[derive(Debug)]
pub enum StoreBuilderError {
    MissingDependecy {
        host: UnitId,
        dep: UnitId
    }
}

impl std::fmt::Display for StoreBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDependecy { host, dep } => {
                write!(f, "Unit '{}' refers to '{}', but '{}' is not defined.", host, dep, dep)
            }
        }
    }
}

#[derive(Debug)]
struct StoreBuilder {
    units: Vec<Unit>
}

impl StoreBuilder {
    fn new() -> Self {
        Self { units: vec![] }
    }

    fn add(mut self, unit: Unit) -> Self {
        self.units.push(unit);
        self
    }

    fn build(self) -> Result<Store, StoreBuilderError> {
        let mut units_map: HashMap<UnitId, Unit> = HashMap::new();

        let known_ids: Vec<UnitId> = self.units.iter().map(|u| u.id.clone()).collect();

        for unit in self.units.into_iter() {
            // Check dependencies
            for dep_id in unit.deps.iter() {
                if !known_ids.contains(dep_id) {
                    let e = StoreBuilderError::MissingDependecy { host: unit.id.clone(), dep: dep_id.clone() };
                    return Err(e);
                }
            }

            // Add to HashMap
            units_map.insert(unit.id.clone(), unit);
        }

        let store = Store { units: units_map };
        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit<S: Into<UnitId>>(id: S) -> Unit {
        unit_with_deps(id, Vec::new())
    }

    fn unit_with_deps<T: Into<UnitId>>(id: T, deps: Vec<T>) -> Unit {
        Unit {
            id: id.into(),
            name: None,
            deps: deps.into_iter().map(|d| d.into()).collect(),
            unit_type: None
        }
    }

    #[test]
    fn test_builder_ok() {
        let store = Store::builder()
            .add(unit("A"))
            .add(unit("B"))
            .build()
            .unwrap();

        let a_id = UnitId::from("A");
        assert_eq!(store.get(&a_id), &unit("A"));

        let b_id = UnitId::from("B");
        assert_eq!(store.get(&b_id), &unit("B"));
    }

    #[test]
    fn test_builder_err() {
        let err = Store::builder()
            .add(unit("A"))
            .add(unit_with_deps("B", vec!["Z"]))
            .build()
            .unwrap_err();

        assert_eq!(
            format!("{}", err),
            "Unit 'B' refers to 'Z', but 'Z' is not defined."
        )
    }
}
