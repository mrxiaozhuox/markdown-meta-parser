use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_till},
    IResult,
};

use crate::items::{MarkInline, MarkItem};

pub fn parse_to_item(md: &str) -> Vec<MarkItem> {
    vec![]
}

fn parse_title(text: &str) -> IResult<&str, MarkItem> {
    // check title #
    let r = is_a("#")(text)?;
    let level = if r.1.len() > 6 { 6 } else { r.1.len() };

    // if title level is 0, not a title
    if level == 0 {
        return Ok((text, MarkItem::Unknown));
    }

    let r = tag(" ")(r.0)?;

    Ok(("", MarkItem::title(level as u8, r.0)))
}

fn parse_quote(text: &str) -> IResult<&str, MarkInline> {
    // check `> ` sign
    let r = tag("> ")(text)?;

    todo!()
}

struct InlineParser;
impl InlineParser {
    pub fn italic_and_bold(md: &str) -> IResult<&str, (MarkInline, MarkInline)> {
        let mut result = (
            MarkInline::Text(String::new()),
            MarkInline::Text(String::new()),
        );

        let mut r = take_till(|c| c == '*' || c == '_' || c == '~' || c == '`')(md)?;

        let mut sign = (&r.0[0..1], 1);
        if sign.0 == "~" {
            sign = ("~", 2);
        } else if (sign.0 == "*" || sign.0 == "_") && &r.0[1..2] == sign.0 {
            sign.1 = 2;
        }
        let r = (&r.0[sign.1..], r.1);

        result.0 = MarkInline::Text(r.1.to_string());

        println!("{:?}", r.0);
        let r = take_till(|c| c == '*' || c == '_' || c == '~' || c == '`')(r.0)?;

        Ok((r.0, result))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_title;
    use crate::{items::MarkItem, parser::InlineParser};

    #[test]
    fn check_title() {
        let r = parse_title("#### hello world");
        assert_eq!(r, Ok(("", MarkItem::title(4, "hello world"))))
    }

    #[test]
    fn test_code() {
        let r = InlineParser::italic_and_bold("hello **Dioxus**");
        println!("{:?}", r);
    }
}
