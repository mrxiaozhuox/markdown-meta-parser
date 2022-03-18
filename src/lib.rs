use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MetaData {
    content: String,
    pub required: Vec<String>,
    pub type_mark: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<String>),
    Bool(bool),
}

impl MetaData {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            required: vec![],
            type_mark: Default::default(),
        }
    }

    pub fn parse(&self) -> anyhow::Result<HashMap<String, Value>> {
        let content = self.content.clone();
        let mut result = HashMap::new();

        let mut started = false;

        for line in content.lines() {
            // if line is empty, just skip
            if line.trim().is_empty() {
                continue;
            }

            if line.trim() == "---" {
                started = !started;
            }

            if started {
                let res = Self::parse_line(line);
                println!("{:?}", res);
            }
        }

        for req in &self.required {
            if !result.contains_key(req) {
                return Err(anyhow::anyhow!("Required key not found: {req}"));
            }
        }

        Ok(result)
    }

    pub fn parse_line(text: &str) -> Option<(String, String)> {
        let mut v = text.split(':').collect::<Vec<&str>>();
        let mut result = (String::new(), String::new());
        if v.len() < 2 {
            return None;
        }

        result.0 = v.get(0).unwrap().trim().into();
        v.remove(0);
        result.1 = v.join(":").trim().into();

        Some(result)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::MetaData;

    #[test]
    fn one_line() {
        let r = MetaData::parse_line("title: Hello World");
        println!("{:?}", r);
    }

    #[test]
    fn test_md() {
        const md: &str = "
---
title: Hello World
author: YuKun Liu
tags: [test]
date: 2003-11-10
---
        ";

        let meta = MetaData {
            content: md.to_string(),
            required: vec![],
            type_mark: HashMap::default(),
        };

        let r = meta.parse();
    }
}
