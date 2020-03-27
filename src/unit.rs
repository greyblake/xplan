#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnitId(String);

#[derive(Debug, PartialEq)]
pub struct UnitType(String);

#[derive(Debug, PartialEq)]
pub struct Unit {
    pub id: UnitId,
    pub name: Option<String>,
    pub deps: Vec<UnitId>,
    pub unit_type: Option<UnitType>
}

impl UnitId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl<T: Into<String>> From<T> for UnitId  {
    fn from(id: T) -> Self {
        UnitId(id.into())
    }
}

impl std::fmt::Display for UnitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
