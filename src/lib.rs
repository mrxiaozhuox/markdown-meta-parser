use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub content: String,
    pub required: Vec<String>,
    pub type_mark: HashMap<String, &'static str>,
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

    pub fn parse(&self) -> anyhow::Result<(HashMap<String, Value>, String)> {
        let content = self.content.clone();
        let mut markdown_text = String::new();
        let mut result = HashMap::new();

        let mut state = 0;

        for line in content.lines() {
            if line.trim() == "---" {
                state += 1;
                if state < 3 {
                    continue;
                }
            } else if state == 0 {
                state = 2;
            }

            if state == 1 {
                if line.trim().is_empty() {
                    continue;
                }

                let res = Self::parse_line(line);
                if let Some((key, value)) = res {
                    if self.type_mark.contains_key(&key) {
                        let target_type = self.type_mark.get(&key).unwrap();
                        let value = match target_type.to_lowercase().as_str() {
                            "bool" => {
                                if &value.to_lowercase() == "true" {
                                    Value::Bool(true)
                                } else if &value.to_lowercase() == "false" {
                                    Value::Bool(false)
                                } else {
                                    Value::String(value)
                                }
                            }
                            "number" => {
                                if let Ok(v) = value.parse::<f64>() {
                                    Value::Number(v)
                                } else {
                                    Value::String(value)
                                }
                            }
                            "array" => {
                                if &value[0..1] == "[" && &value[value.len() - 1..] == "]" {
                                    let mut array = vec![];
                                    let temp = value[1..value.len() - 1].to_string();
                                    let v = temp.split(',').collect::<Vec<&str>>();
                                    for i in v {
                                        array.push(i.trim().to_string());
                                    }
                                    Value::Array(array)
                                } else {
                                    Value::String(value)
                                }
                            }
                            _ => Value::String(value),
                        };
                        result.insert(key, value);
                    } else {
                        result.insert(key, Value::String(value));
                    }
                }
            } else {
                markdown_text += &format!("{line}\n");
            }
        }

        for req in &self.required {
            if !result.contains_key(req) {
                return Err(anyhow::anyhow!("Required key not found: {req}"));
            }
        }

        Ok((result, markdown_text))
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
    use crate::MetaData;
    #[test]
    fn one_line() {
        let r = MetaData::parse_line("title: Hello World");
        println!("{:?}", r);
    }
}
