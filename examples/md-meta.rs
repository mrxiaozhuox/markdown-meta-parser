use std::collections::HashMap;

fn main() {
    let content = include_str!("./test.md").to_string();

    let mut type_mark = HashMap::new();

    type_mark.insert("tags".into(), "array");
    type_mark.insert("released".into(), "bool");

    let meta = markdown_meta_parser::MetaData {
        content,
        required: vec!["title".into()],
        type_mark,
    };

    println!("{:#?}", meta.parse());
}
