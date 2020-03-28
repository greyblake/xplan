use yaml_rust::{yaml::Yaml};

use super::error::ParseError;

pub fn parse_yaml_to_string(val: &Yaml) -> Result<String, ParseError> {
    match val {
        Yaml::String(s) => Ok(s.to_owned()),
        Yaml::Integer(num) => Ok(num.to_string()),
        _ => {
            let message = format!("Expected a string. Got {:?}", val);
            Err(ParseError::Base(message))
        }
    }
}

pub fn parse_yaml_to_vec(val: &Yaml) -> Result<Vec<String>, ParseError> {
    let mut items: Vec<String> = vec![];

    match val {
        Yaml::Array(arr) => {
            for yaml_item in arr.iter() {
                let item = parse_yaml_to_string(yaml_item)?;
                items.push(item);
            }
        }
        _ => {
            let message = format!("Value must be an array. Got {:?}", val);
            return Err(ParseError::Base(message));
        }
    }

    Ok(items)
}
