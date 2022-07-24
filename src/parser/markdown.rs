use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, digit1},
    combinator::map,
    multi::{many0, many1},
    sequence::delimited,
    sequence::terminated,
    Err,
};

use super::error::*;
use super::utils::*;
use crate::ast::*;

pub fn page(input: Span) -> IResult<Page> {
    let (input, nodes) = many0(alt((
        // parser for multiline block
        map(code_block, |s| Node::new(NodeKind::CodeBlock(s))),
        map(table, |s| Node::new(NodeKind::Table(s))),
        map(list, |s| Node::new(NodeKind::List(s))),
        map(paragraph, |s| Node::new(NodeKind::Paragraph(s))),
        // workaround for no-newline like "hoge"
        // map(text, |s| Node::new(NodeKind::Text(s))),
        node,
    )))(input)?;
    Ok((input, Page { nodes }))
}

// "hoge\n"
fn paragraph(input: Span) -> IResult<Paragraph> {
    let (input, p) = take_until_eol(input)?;
    let (input, _) = char('\n')(input)?;
    let (rest, p) = map(many0(node), |children| Paragraph::new(children))(p)?;
    assert!(rest.is_empty());
    Ok((input, p))
}

fn list(input: Span) -> IResult<List> {
    map(many1(list_item), |children| List::new(children))(input)
}

fn node(input: Span) -> IResult<Node> {
    alt((
        // parser for single line
        map(heading, |c| Node::new(NodeKind::Heading(c))),
        map(hashtag, |s| Node::new(NodeKind::HashTag(s))),
        map(block_quate, |s| Node::new(NodeKind::BlockQuate(s))),
        map(image, |s| Node::new(NodeKind::Image(s))),
        map(emphasis, |c| Node::new(NodeKind::Emphasis(c))),
        map(external_link, |c| Node::new(NodeKind::ExternalLink(c))),
        map(math, |c| Node::new(NodeKind::Math(c))),
        // NOTE(tkat0): keep internal_link at the bottom of parsing bracket node
        map(internal_link, |c| Node::new(NodeKind::InternalLink(c))),
        map(external_link_plain, |s| {
            Node::new(NodeKind::ExternalLink(s))
        }),
        map(text, |s| Node::new(NodeKind::Text(s))),
    ))(input)
}

// [[internal link]]
fn internal_link(input: Span) -> IResult<InternalLink> {
    map(
        delimited(tag("[["), take_while(|c| c != ']'), tag("]]")),
        |s: Span| InternalLink::new(*s),
    )(input)
}

/// [Rust](https://www.rust-lang.org/)
fn external_link(input: Span) -> IResult<ExternalLink> {
    let (input, title) = brackets(input)?;
    let (input, url) = parentheses(input)?;

    Ok((input, ExternalLink::new(Some(*title), *url)))
}

/// ![](https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png)
/// ![image](https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png)
fn image(input: Span) -> IResult<Image> {
    let ext = ["svg", "jpg", "jpeg", "png", "gif"];
    let (input, _) = char('!')(input)?;
    let (input, title) = brackets(input)?;
    let (input, url) = parentheses(input)?;

    let is_image = |url: &str| ext.iter().any(|e| url.ends_with(e));

    if is_image(*url) {
        return Ok((input, Image::new(*url)));
    } else {
        Err(Err::Error(ParseError::new(
            input,
            "URL is not image".into(),
        )))
    }
}

fn heading(input: Span) -> IResult<Heading> {
    let (input, hash) = many1(tag("#"))(input)?;
    let level = hash.len();
    let (input, _) = char(' ')(input)?;
    map(take_until_eol, move |s: Span| Heading::new(*s, level))(input)
}

// TODO(tkat0): mix is not supported yet
fn emphasis(input: Span) -> IResult<Emphasis> {
    alt((bold, italic, strikethrough))(input)
}

/// **bold**
fn bold(input: Span) -> IResult<Emphasis> {
    map(
        delimited(tag("**"), take_while(|c| c != '*'), tag("**")),
        |s: Span| Emphasis::bold(*s),
    )(input)
}

/// *italic*
fn italic(input: Span) -> IResult<Emphasis> {
    map(
        delimited(tag("*"), take_while(|c| c != '*'), tag("*")),
        |s: Span| Emphasis::italic(*s),
    )(input)
}

/// ~~strikethrough~~
fn strikethrough(input: Span) -> IResult<Emphasis> {
    map(
        delimited(tag("~~"), take_while(|c| c != '~'), tag("~~")),
        |s: Span| Emphasis::strikethrough(*s),
    )(input)
}

/// $$ Tex here $$
fn math(input: Span) -> IResult<Math> {
    map(
        delimited(tag("$$"), take_while(|c| c != '$'), tag("$$")),
        |s: Span| Math::new(*s),
    )(input)
}

/// ```hello.rs
/// ```
///
fn code_block(input: Span) -> IResult<CodeBlock> {
    let (input, _) = tag("```")(input)?;
    let (input, file_name) = take_until("\n")(input)?;
    let (input, _) = char('\n')(input)?;

    let (input, block) = take_until("```")(input)?;
    // TODO: "\n" needed
    let (input, _) = tag("```\n")(input)?;

    let (rest, block) = map(
        many0(terminated(take_while(|c| c != '\n'), char('\n'))),
        move |codes: Vec<Span>| {
            CodeBlock::new(*file_name, codes.iter().map(|span| **span).collect())
        },
    )(block)?;
    assert!(rest.is_empty());
    Ok((input, block))
}

/// | a | b | c |
/// | --- | --- | --- |
/// | d | e | f |
fn table(input: Span) -> IResult<Table> {
    fn row(input: Span) -> IResult<Vec<String>> {
        let (rest, input) = take_until_eol(input)?;
        let (rest, _) = char('\n')(rest)?;
        let (input, _) = char('|')(input)?;
        let (input, row) = many1(terminated(take_until("|"), tag("|")))(input)?;
        assert!(input.is_empty());
        let row = row.into_iter().map(|s| s.trim().to_string()).collect();
        Ok((rest, row))
    }

    let (input, header) = row(input)?;
    let (input, _) = row(input)?;
    let (input, rows) = many0(row)(input)?;

    Ok((input, Table::new("table", header, rows)))
}

fn list_item(input: Span) -> IResult<ListItem> {
    // TODO(tkat0): indent size and variation
    let (input, tabs) = many0(tag("  "))(input)?;
    let level = tabs.len();

    fn decimal(input: Span) -> IResult<(ListKind, Vec<Node>)> {
        let (input, _) = terminated(digit1, tag(". "))(input)?;
        let (input, children) = many0(node)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, (ListKind::Decimal, children)))
    }

    fn disc(input: Span) -> IResult<(ListKind, Vec<Node>)> {
        let (input, _) = alt((tag("* "), tag("- ")))(input)?;
        let (input, children) = many0(node)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, (ListKind::Disc, children)))
    }

    map(alt((decimal, disc)), move |(kind, children)| {
        ListItem::new(kind, level, children)
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest(input, expected,
        case("    * 123abc\n", ("", List::new(vec![ListItem::new(ListKind::Disc, 2, vec![Node::new(NodeKind::Text(Text::new("123abc")))])]))),
        case("* 123abc\n", ("", List::new(vec![ListItem::new(ListKind::Disc, 0, vec![Node::new(NodeKind::Text(Text::new("123abc")))])]))),
        case("  123. abc\n", ("", List::new(vec![ListItem::new(ListKind::Decimal, 1, vec![Node::new(NodeKind::Text(Text::new("abc")))])]))),
    )]
    fn list_valid_test(input: &str, expected: (&str, List)) {
        assert_eq!(
            list(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("```hello.rs\n    panic!()\n    panic!()\n```\n", ("", CodeBlock::new("hello.rs", vec!["    panic!()", "    panic!()"]))),
    )]
    fn code_block_valid_test(input: &str, expected: (&str, CodeBlock)) {
        assert_eq!(
            code_block(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("| a | b | c |\n| --- | --- | --- |\n", ("", Table::new("table", vec!["a".into(), "b".into(), "c".into()], vec![]))),
        case("| a | b | c |\n| --- | --- | --- |\n| d | e | f |\n", ("", Table::new("table", vec!["a".into(), "b".into(), "c".into()], vec![vec!["d".into(), "e".into(), "f".into()]]))),
    )]
    fn table_valid_test(input: &str, expected: (&str, Table)) {
        assert_eq!(
            table(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("![](https://www.rust-lang.org/static/images/rust-logo-blk.svg)", ("", Image::new("https://www.rust-lang.org/static/images/rust-logo-blk.svg"))),
        case("![title](https://www.rust-lang.org/static/images/rust-logo-blk.svg)", ("", Image::new("https://www.rust-lang.org/static/images/rust-logo-blk.svg"))),
    )]
    fn image_valid_test(input: &str, expected: (&str, Image)) {
        assert_eq!(
            image(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("# heading", ("", Heading::new("heading", 1))),
        case("## heading", ("", Heading::new("heading", 2))),
    )]
    fn heading_valid_test(input: &str, expected: (&str, Heading)) {
        assert_eq!(
            heading(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("**text**", ("", Emphasis::bold_level("text", 1))),
        case("*text*", ("", Emphasis::italic("text"))),
        case("~~text~~", ("", Emphasis::strikethrough("text"))),
        // case("~~***text***~~", ("", Emphasis::new("text", 1, 1, 1))),
    )]
    fn emphasis_valid_test(input: &str, expected: (&str, Emphasis)) {
        assert_eq!(
            emphasis(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[[title]]", ("", InternalLink::new("title"))),
    )]
    fn internal_link_valid_test(input: &str, expected: (&str, InternalLink)) {
        assert_eq!(
            internal_link(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[Rust](https://www.rust-lang.org/)", ("", ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/"))),
        case("[Rustプログラミング言語](https://www.rust-lang.org/)", ("", ExternalLink::new(Some("Rustプログラミング言語"), "https://www.rust-lang.org/"))),
    )]
    fn external_link_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case(r#"$$ \frac{-b \pm \sqrt{b^2-4ac}}{2a} $$"#, ("", Math::new(r#" \frac{-b \pm \sqrt{b^2-4ac}}{2a} "#))),
    )]
    fn math_valid_test(input: &str, expected: (&str, Math)) {
        assert_eq!(
            math(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("abc #tag ", ("#tag ", Node::new(NodeKind::Text(Text::new("abc "))))),
        case("[[title]]abc", ("abc", Node::new(NodeKind::InternalLink(InternalLink::new("title"))))),
        case("[", ("", Node::new(NodeKind::Text(Text::new("["))))),
        case(r#"$$ \frac{-b \pm \sqrt{b^2-4ac}}{2a} $$"#, ("", Node::new(NodeKind::Math(Math::new(r#" \frac{-b \pm \sqrt{b^2-4ac}}{2a} "#))))),
    )]
    fn node_valid_test(input: &str, expected: (&str, Node)) {
        assert_eq!(
            node(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, case(""), case("\n"))]
    fn node_invalid_test(input: &str) {
        if let Ok(ok) = text(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }

    /* TODO: fix
    #[rstest(input, expected,
        case(" ", ("", Paragraph::new( vec![Node::new(NodeKind::Text(Text::new(" ")))]))),
        case("", ("", Paragraph::new( vec![]))),
        case("#tag #tag [[internal link]]", ("", Paragraph::new(
            vec![
                Node::new(NodeKind::HashTag(HashTag::new("tag"))),
                Node::new(NodeKind::Text(Text::new(" "))),
                Node::new(NodeKind::HashTag(HashTag::new("tag"))),
                Node::new(NodeKind::Text(Text::new(" "))),
                Node::new(NodeKind::InternalLink(InternalLink::new("internal link")))
                ]
            ))),
    )]
    fn paragraph_valid_test(input: &str, expected: (&str, Paragraph)) {
        assert_eq!(
            paragraph(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }
     */

    #[rstest(input, expected,
        case(indoc! {"
            abc
            #efg [[internal link]][Rust](https://www.rust-lang.org/)
        "}, ("", Page {
            nodes: vec![
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![
                        Node::new(NodeKind::Text(Text {
                        value: "abc".into(),
                    }))
                ]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![
                        Node::new(NodeKind::HashTag(HashTag {
                            value: "efg".into(),
                        })),
                        Node::new(NodeKind::Text(Text { value: " ".into() })),
                        Node::new(NodeKind::InternalLink(InternalLink::new("internal link"))),
                        Node::new(NodeKind::ExternalLink(ExternalLink::new(
                            Some("Rust"),
                            "https://www.rust-lang.org/",
                        ))),
                ]))),
            ]
        })),
        case(indoc! {"
            a

            b
        "}, ("", Page {
            nodes: vec![
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![
                        Node::new(NodeKind::Text(Text {
                        value: "a".into(),
                    }))
                ]))),
                                Node::new(NodeKind::Paragraph(Paragraph::new(vec![]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![
                        Node::new(NodeKind::Text(Text {
                        value: "b".into(),
                    }))
                ]))),
            ]
        }))
    )]
    fn page_valid_test(input: &str, expected: (&str, Page)) {
        assert_eq!(
            page(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }
}
