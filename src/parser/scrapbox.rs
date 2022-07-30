use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, digit1},
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::delimited,
    sequence::terminated,
    Err, InputTake, Slice,
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

fn paragraph(input: Span) -> IResult<Paragraph> {
    map(terminated(many0(node), char('\n')), |children| {
        Paragraph::new(children)
    })(input)
}

fn list(input: Span) -> IResult<List> {
    map(many1(list_item), |children| List::new(children))(input)
}

fn node(input: Span) -> IResult<Node> {
    alt((
        // parser for single line
        map(hashtag, |s| Node::new(NodeKind::HashTag(s))),
        map(block_quate, |s| Node::new(NodeKind::BlockQuate(s))),
        map(image, |s| Node::new(NodeKind::Image(s))),
        map(emphasis, |c| Node::new(NodeKind::Emphasis(c))),
        map(bold, |c| Node::new(NodeKind::Emphasis(c))),
        map(external_link, |c| Node::new(NodeKind::ExternalLink(c))),
        map(math, |c| Node::new(NodeKind::Math(c))),
        map(external_link_other_project, |s| {
            Node::new(NodeKind::ExternalLink(s))
        }),
        // NOTE(tkat0): keep internal_link at the bottom of parsing bracket node
        map(internal_link, |c| Node::new(NodeKind::InternalLink(c))),
        map(external_link_plain, |s| {
            Node::new(NodeKind::ExternalLink(s))
        }),
        map(commandline, |s| Node::new(NodeKind::BlockQuate(s))),
        map(text, |s| Node::new(NodeKind::Text(s))),
    ))(input)
}

// [internal link]
fn internal_link(input: Span) -> IResult<InternalLink> {
    let (input, text) = brackets(input)?;
    Ok((input, InternalLink::new(*text)))
}

// [/help-jp/Scrapbox]
fn external_link_other_project(input: Span) -> IResult<ExternalLink> {
    map(
        delimited(tag("[/"), take_while(|c| c != ']'), char(']')),
        |s: Span| ExternalLink::new(None, &format!("https://scrapbox.io/{}", *s)),
    )(input)
}

/// [https://www.rust-lang.org/] or [https://www.rust-lang.org/ Rust] or [Rust https://www.rust-lang.org/]
fn external_link(input: Span) -> IResult<ExternalLink> {
    let (input, text) = brackets(input)?;

    // [https://www.rust-lang.org/ Rust] or [https://www.rust-lang.org/]
    fn url_title(input: Span) -> IResult<ExternalLink> {
        let (input, _) = space0(input)?;
        let (input, url) = url(input)?;
        let (title, _) = space0(input)?;
        let title = if title.is_empty() { None } else { Some(title) };
        Ok((
            Span::new(""),
            ExternalLink::new(title.map(|s: Span| *s), &url),
        ))
    }

    // [Rust https://www.rust-lang.org/]
    // [Rust Rust Rust https://www.rust-lang.org/]
    // [Rust　Rust　Rust　https://www.rust-lang.org/]
    // [Rust https://www.rust-lang.org/ https://www.rust-lang.org/]
    fn title_url(input: Span) -> IResult<ExternalLink> {
        let (link, title) = {
            let mut index = 0;
            for i in (0..input.len()).rev() {
                let e = input.slice(i..i + 1);
                if *e == " " || *e == "　" {
                    index = i;
                    break;
                }
            }
            input.take_split(index)
        };

        let title = if title.is_empty() { None } else { Some(title) };
        let (input, _) = space1(link)?;
        let (rest, url) = url(input)?;
        assert!(rest.is_empty());
        Ok((rest, ExternalLink::new(title.map(|s: Span| *s), &url)))
    }

    let (rest, link) = alt((url_title, title_url))(text)?;
    assert!(rest.is_empty());
    Ok((input, link))
}

/// [http://cutedog.com https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
/// [https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
fn image(input: Span) -> IResult<Image> {
    let ext = ["svg", "jpg", "jpeg", "png", "gif"];
    let (input, text) = brackets(input)?;
    let (text, url1) = url(text)?;

    let is_image = |url: &str| ext.iter().any(|e| url.ends_with(e));

    if text.is_empty() && is_image(&url1) {
        // [https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
        return Ok((input, Image::new(&url1)));
    }

    let (text, _) = space1(text)?;

    let (text, url2) = url(text)?;
    if text.is_empty() && is_image(&url2) {
        // [http://cutedog.com https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
        return Ok((input, Image::new(&url2)));
    } else {
        Err(Err::Error(ParseError::new(input, "".into())))
    }
}

/// [/icons/todo.icon]
fn icon() {}

/// [*-/** emphasis]
/// [[Bold]] or [* Bold] or [*** Bold]
/// [/ italic]
/// [- strikethrough]
fn emphasis(input: Span) -> IResult<Emphasis> {
    let (input, text) = brackets(input)?;

    let (rest, tokens) = take_while(|c| ['*', '/', '-'].contains(&c))(text)?;
    let (text, _) = char(' ')(rest)?;

    let mut bold = 0;
    let mut italic = 0;
    let mut strikethrough = 0;
    for c in tokens.chars() {
        match &c {
            '*' => bold += 1,
            '/' => italic += 1,
            '-' => strikethrough += 1,
            _ => {}
        }
    }

    Ok((
        input,
        Emphasis::new(text.trim(), bold, italic, strikethrough),
    ))
}

// [[bold]]
fn bold(input: Span) -> IResult<Emphasis> {
    map(
        delimited(tag("[["), take_while(|c| c != ']'), tag("]]")),
        |s: Span| Emphasis::bold(s.trim()),
    )(input)
}

/// [$ Tex here]
fn math(input: Span) -> IResult<Math> {
    map(
        delimited(tag("[$"), take_while(|c| c != ']'), char(']')),
        |s: Span| Math::new(s.trim()),
    )(input)
}

/// code:filename.extension
/// TODO(tkat0): List + CodeBlock is not supported yet.
fn code_block(input: Span) -> IResult<CodeBlock> {
    let (input, _) = tag("code:")(input)?;
    let (input, file_name) = take_until("\n")(input)?;
    let (input, _) = char('\n')(input)?;
    map(
        many0(delimited(char(' '), take_while(|c| c != '\n'), char('\n'))),
        move |codes: Vec<Span>| {
            CodeBlock::new(
                *file_name,
                codes.iter().map(|span| *span.fragment()).collect(),
            )
        },
    )(input)
}

/// table:name\n
///  A\tB\tC\n
fn table(input: Span) -> IResult<Table> {
    let (input, _) = tag("table:")(input)?;
    let (input, name) = take_until("\n")(input)?;
    let (input, _) = char('\n')(input)?;

    fn row(input: Span) -> IResult<Vec<String>> {
        let (input, text) = delimited(char(' '), take_while(|c| c != '\n'), char('\n'))(input)?;

        fn take_until_t(input: Span) -> IResult<String> {
            let (input, value) = take_until("\t")(input)?;
            let (input, _) = tag("\t")(input)?;
            Ok((input, value.to_string()))
        }

        fn take_until_n(input: Span) -> IResult<String> {
            let (input, value) = take_until_eol(input)?;
            Ok((input, value.to_string()))
        }

        let (text, mut x) = many0(take_until_t)(text)?;
        let (text, y) = take_until_n(text)?;

        if !y.is_empty() {
            x.push(y);
        }
        assert!(text.is_empty());

        Ok((input, x))
    }

    let (input, header) = opt(row)(input)?;

    if let Some(header) = header {
        let (input, rows) = many0(row)(input)?;
        Ok((input, Table::new(*name, header, rows)))
    } else {
        Ok((input, Table::new(*name, vec![], vec![])))
    }
}

/// >
fn quote() {}

/// $ hoge or % hoge
fn commandline(input: Span) -> IResult<BlockQuate> {
    let (input, prefix) = alt((tag("$ "), tag("% ")))(input)?;
    let prefix = prefix.to_string();
    let (input, text) = take_until_eol(input)?;
    Ok((input, BlockQuate::new(&format!("{}{}", prefix, text))))
}

/// ? hoge
fn helpfeel() {}

/// <tab>
/// <tab>1.
fn list_item(input: Span) -> IResult<ListItem> {
    let (input, tabs) = many1(char('\t'))(input)?;
    let (input, decimal) = opt(terminated(digit1, tag(". ")))(input)?;
    let kind = match &decimal {
        Some(_) => ListKind::Decimal,
        None => ListKind::Disc,
    };
    let (input, children) = many0(node)(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, ListItem::new(kind, tabs.len(), children)))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest(input, expected,
        case("\t\t123abc\n", ("", List::new(vec![ListItem::new(ListKind::Disc, 2, vec![Node::new(NodeKind::Text(Text::new("123abc")))])]))),
        case("\t123. abc\n", ("", List::new(vec![ListItem::new(ListKind::Decimal, 1, vec![Node::new(NodeKind::Text(Text::new("abc")))])]))),
    )]
    fn list_valid_test(input: &str, expected: (&str, List)) {
        assert_eq!(
            list(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("$ code   ", ("", BlockQuate::new("$ code   "))),
    )]
    fn commandline_valid_test(input: &str, expected: (&str, BlockQuate)) {
        assert_eq!(
            commandline(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("code:hello.rs\n     panic!()\n     panic!()\n", ("", CodeBlock::new("hello.rs", vec!["    panic!()", "    panic!()"]))),
    )]
    fn code_block_valid_test(input: &str, expected: (&str, CodeBlock)) {
        assert_eq!(
            code_block(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("table:table\n", ("", Table::new("table", vec![], vec![]))),
        case("table:table\n a\tb\tc\n d\te\tf\n", ("", Table::new("table", vec!["a".into(), "b".into(), "c".into()], vec![vec!["d".into(), "e".into(), "f".into()]]))),
        // case("table:table\n a\tb\tc\n", ("", Table::new("table", vec!["a".into(), "b".into(), "c".into()], vec![vec![]]))),
    )]
    fn table_valid_test(input: &str, expected: (&str, Table)) {
        assert_eq!(
            table(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[https://www.rust-lang.org/static/images/rust-logo-blk.svg]", ("", Image::new("https://www.rust-lang.org/static/images/rust-logo-blk.svg"))),
        // TODO(tkat0): enable link
        case("[https://www.rust-lang.org/ https://www.rust-lang.org/static/images/rust-logo-blk.svg]", ("", Image::new("https://www.rust-lang.org/static/images/rust-logo-blk.svg"))),
        case("[https://www.rust-lang.org/　https://www.rust-lang.org/static/images/rust-logo-blk.svg]", ("", Image::new("https://www.rust-lang.org/static/images/rust-logo-blk.svg"))),
    )]
    fn image_valid_test(input: &str, expected: (&str, Image)) {
        assert_eq!(
            image(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[* text]", ("", Emphasis::bold_level("text", 1))),
        case("[***** text]", ("", Emphasis::bold_level("text", 5))),
        case("[/ text]", ("", Emphasis::italic("text"))),
        case("[*/*-* text]", ("", Emphasis::new("text", 3, 1, 1))),
        case("[*/*-*  text　]", ("", Emphasis::new("text", 3, 1, 1))),
    )]
    fn emphasis_valid_test(input: &str, expected: (&str, Emphasis)) {
        assert_eq!(
            emphasis(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[[text]]", ("", Emphasis::bold_level("text", 1))),
        case("[[ text　]]", ("", Emphasis::bold_level("text", 1))),
    )]
    fn bold_valid_test(input: &str, expected: (&str, Emphasis)) {
        assert_eq!(
            bold(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[title]", ("", InternalLink::new("title"))),
    )]
    fn internal_link_valid_test(input: &str, expected: (&str, InternalLink)) {
        assert_eq!(
            internal_link(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[/help-jp/Scrapbox]", ("", ExternalLink::new(None, "https://scrapbox.io/help-jp/Scrapbox"))),
    )]
    fn external_link_other_project_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link_other_project(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[https://www.rust-lang.org/]", ("", ExternalLink::new(None, "https://www.rust-lang.org/"))),
        case("[  https://www.rust-lang.org/]", ("", ExternalLink::new(None, "https://www.rust-lang.org/"))),
        case("[  https://www.rust-lang.org/  ]", ("", ExternalLink::new(None, "https://www.rust-lang.org/"))),
        case("[Rust https://www.rust-lang.org/]", ("", ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/"))),
        case("[Rust  https://www.rust-lang.org/]", ("", ExternalLink::new(Some("Rust "), "https://www.rust-lang.org/"))),
        case("[https://www.rust-lang.org/ Rust]", ("", ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/"))),
        case("[https://www.rust-lang.org/  Rust]", ("", ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/"))),
        case("[https://www.rust-lang.org/  Rust Rust Rust]", ("", ExternalLink::new(Some("Rust Rust Rust"), "https://www.rust-lang.org/"))),
        case("[Rust Rust Rust https://www.rust-lang.org/]", ("", ExternalLink::new(Some("Rust Rust Rust"), "https://www.rust-lang.org/"))),
        case("[https://www.rust-lang.org/]\n[*-/ text]", ("\n[*-/ text]", ExternalLink::new(None, "https://www.rust-lang.org/"))),
        case("[Rustプログラミング言語 https://www.rust-lang.org/]", ("", ExternalLink::new(Some("Rustプログラミング言語"), "https://www.rust-lang.org/"))),
        // Scrapbox actually doesn't parse this
        case("[ https://www.rust-lang.org/ Rust ]", ("", ExternalLink::new(Some("Rust "), "https://www.rust-lang.org/"))),
        case("[Rust https://www.rust-lang.org/ https://www.rust-lang.org/]", ("", ExternalLink::new(Some("Rust https://www.rust-lang.org/"), "https://www.rust-lang.org/"))),
    )]
    fn external_link_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case(r#"[$ \frac{-b \pm \sqrt{b^2-4ac}}{2a} ]"#, ("", Math::new(r#"\frac{-b \pm \sqrt{b^2-4ac}}{2a}"#))),
    )]
    fn math_valid_test(input: &str, expected: (&str, Math)) {
        assert_eq!(
            math(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("abc #tag ", ("#tag ", Node::new(NodeKind::Text(Text::new("abc "))))),
        case("[title]abc", ("abc", Node::new(NodeKind::InternalLink(InternalLink::new("title"))))),
        case("[", ("", Node::new(NodeKind::Text(Text::new("["))))),
        case(r#"[$ \frac{-b \pm \sqrt{b^2-4ac}}{2a} ]"#, ("", Node::new(NodeKind::Math(Math::new(r#"\frac{-b \pm \sqrt{b^2-4ac}}{2a}"#))))),
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

    #[rstest(input, expected,
        case(" \n", ("", Paragraph::new( vec![Node::new(NodeKind::Text(Text::new(" ")))]))),
        case("#tag #tag [internal link]\n", ("", Paragraph::new(
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

    #[rstest(input, case(" "), case(""))]
    fn paragraph_invalid_test(input: &str) {
        if let Ok(ok) = paragraph(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }

    #[rstest(input, expected,
        case(indoc! {"
            abc
            #efg [internal link][https://www.rust-lang.org/]
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
                            None,
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
