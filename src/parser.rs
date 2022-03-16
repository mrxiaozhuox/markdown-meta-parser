use nom::{
    bytes::complete::{is_a, tag, take_till},
    IResult,
};

use crate::items::{MarkItem, MarkInline};

pub fn parse_to_item(md: &str) -> Vec<MarkItem> {
    vec![]
}

pub fn parse_inline(md: &str) -> IResult<&str, Vec<MarkInline>> {
    todo!()
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

fn parse_quote(text: &str) -> IResult<&str, MarkItem> {
    // check `> ` sign
    let r = tag("> ")(text)?;

    // Ok(("", MarkItem::))
}

#[cfg(test)]
mod tests {
    use super::parse_title;
    use crate::items::MarkItem;

    #[test]
    fn check_title() {
        let r = parse_title("#### hello world");
        assert_eq!(r, Ok(("", MarkItem::title(4, "hello world"))))
    }
}
