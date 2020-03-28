use thiserror::Error;

use crate::store::StoreBuilderError;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid YAML")]
    InvalidYaml(#[from] yaml_rust::scanner::ScanError),

    #[error("{0}")]
    Base(String),

    #[error("Unknown root element: {0}")]
    UnkownRootElement(String),

    #[error("failed to build")]
    Build(#[from] StoreBuilderError)
}
