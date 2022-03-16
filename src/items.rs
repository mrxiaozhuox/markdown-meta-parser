#[derive(Debug, PartialEq, Clone)]
pub enum MarkItem {
    Text(String),

    Paragraph(Vec<MarkInline>),

    Title((u8, String)),

    Code(M_Code),
    Quote(Vec<MarkInline>),
    Link(M_Link),
    Image(M_Image),

    UnorderedList(Vec<Vec<MarkInline>>),
    OrderedList(Vec<Vec<MarkInline>>),

    Unknown,
}

impl MarkItem {
    pub fn title(level: u8, content: &str) -> Self {
        Self::Title((level, content.to_string()))
    }
    pub fn quote(content: Vec<MarkInline>) -> Self {
        Self::Quote(content)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct M_Code {
    language: String,
    content: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct M_Link {
    to: String,
    content: Vec<MarkItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct M_Image {
    title: String,
    source: String,
}


#[derive(Debug, PartialEq, Clone)]
pub enum MarkInline {
    Italic(String),
    Strong(String),
    Strikethrough(String),
}