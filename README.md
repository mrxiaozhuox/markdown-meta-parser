# Markdown Meta Parser

Parse Markdown Metadata


```markdown
---
title: Hello World
author: mrxiaozhuox<mrxzx.info@gmail.com>
tags: [post, test]
released: false
---

# This is my first post!

Hello World!!
```

```rust

let content = String::from("....");

let mut type_mark = HashMap::new();

type_mark.insert("tags".into(), "array");
type_mark.insert("released".into(), "bool");


let meta = MetaData {
    content,
    required: vec!["title".to_string()],
    type_mark,
}

println!("{:#?}", meta.parse().unwrap());
```

```text
(
    {
        "author": String(
            "mrxiaozhuox<mrxzx.info@gmail.com>",
        ),
        "tags": Array(
            [
                "post",
                "test",
            ],
        ),
        "released": Bool(
            false,
        ),
        "title": String(
            "Hello World",
        ),
    },
    "\n# This is my first post!\n\nHello World!!\n",
),
```