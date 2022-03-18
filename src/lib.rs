use std::collections::HashMap;

use nom::{bytes::complete::take_till1, IResult};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct MetaData {
    result: Value,
    content: String,
    config: MetaDataConfig,
}

#[derive(Debug, Clone, Default)]
pub struct MetaDataConfig {
    pub required: Vec<String>,
    pub type_mark: HashMap<String, TypeMark>,
}

#[derive(Debug, Clone)]
pub enum TypeMark {
    String,
    Number,
    Array,
    Bool,
}

impl MetaData {
    pub fn new(content: &str, config: MetaDataConfig) -> Self {
        Self {
            result: Value::Object(Map::new()),
            content: content.to_string(),
            config,
        }
    }

    pub fn parse_line(text: &str) -> IResult<&str, (&str, &str)> {
        let r = take_till1(|c| c == ':')(text)?;

        let key = r.1.trim();
        let value = r.0[1..].trim();

        // empty str, (key, value)
        Ok(("", (key, value)))
    }
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            result: Value::Null,
            content: Default::default(),
            config: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MetaData;

    #[test]
    fn one_line() {
        let _meta = MetaData::default();
        let r = MetaData::parse_line("title : Hello World");
        println!("{:?}", r);
    }
}
