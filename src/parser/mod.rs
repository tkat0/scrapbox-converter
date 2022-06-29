use std::convert::identity;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while},
    character::complete::{char, digit1},
    combinator::{map, opt, peek},
    multi::{many0, many1},
    sequence::terminated,
    sequence::{delimited, preceded},
    Err, Slice,
};

use crate::ast::*;

mod error;
mod utils;
pub use error::*;
use utils::*;

pub fn page(input: Span) -> IResult<Page> {
    let (input, lines) = many0(line)(input)?;

    Ok((input, Page { lines }))
}

fn line(input: Span) -> IResult<Line> {
    if input.is_empty() {
        return Err(Err::Error(ParseError::new(input, "line is empty".into())));
    }

    // skip '\n' if it is at the beginning of the line.
    let (input, _) = opt(char('\n'))(input)?;

    let (input, list) = list(input)?;

    if let Some(list) = &list {
        map(many0(expr), |c| {
            Line::new(
                LineKind::List(list.clone()),
                c.into_iter().filter_map(identity).collect(),
            )
        })(input)
    } else {
        map(many0(expr), |c| {
            Line::new(
                LineKind::Normal,
                c.into_iter().filter_map(identity).collect(),
            )
        })(input)
    }
}

fn expr(input: Span) -> IResult<Option<Expr>> {
    map(
        alt((
            map(hashtag, |s| Expr::new(ExprKind::HashTag(s))),
            map(block_quate, |s| Expr::new(ExprKind::BlockQuate(s))),
            map(code_block, |s| Expr::new(ExprKind::CodeBlock(s))),
            map(table, |s| Expr::new(ExprKind::Table(s))),
            map(image, |s| Expr::new(ExprKind::Image(s))),
            map(emphasis, |c| Expr::new(ExprKind::Emphasis(c))),
            map(bold, |c| Expr::new(ExprKind::Emphasis(c))),
            map(external_link, |c| Expr::new(ExprKind::ExternalLink(c))),
            map(external_link_other_project, |s| {
                Expr::new(ExprKind::ExternalLink(s))
            }),
            // NOTE(tkat0): keep internal_link at the bottom of parsing bracket expr
            map(internal_link, |c| Expr::new(ExprKind::InternalLink(c))),
            map(external_link_plain, |s| {
                Expr::new(ExprKind::ExternalLink(s))
            }),
            map(commandline, |s| Expr::new(ExprKind::BlockQuate(s))),
            map(text, |s| Expr::new(ExprKind::Text(s))),
        )),
        Some,
    )(input)
}

/// #tag
fn hashtag(input: Span) -> IResult<HashTag> {
    let terminators = vec![" ", "　", "\n"];

    // TODO(tkat0): "#[tag]" -> Error
    //  it should be handled with text + internal link

    map(
        preceded(
            tag("#"),
            take_while(move |c: char| !terminators.contains(&c.to_string().as_str())),
        ),
        |s: Span| HashTag::new(s.fragment()),
    )(input)
}

fn text(input: Span) -> IResult<Text> {
    if input.is_empty() {
        return Err(Err::Error(ParseError::new(input, "".into())));
    }

    if input.starts_with("#") {
        return Err(Err::Error(ParseError::new(input, "".into())));
    }

    if let (rest, Some(value)) = opt(tag("["))(input)? {
        return Ok((rest, Text::new(*value)));
    }

    // "abc #tag" -> ("#tag", "abc ")
    fn take_until_tag(input: Span) -> IResult<Span> {
        // " #tag" -> ("#tag", " ")
        // allow "abc#tag"
        let (input, _) = peek(take_until(" #"))(input)?;
        take_until("#")(input)
    }

    // "abc \n" -> ("", "abc ")
    fn take_until_newline(input: Span) -> IResult<Span> {
        let (input, text) = take_until("\n")(input)?;
        // let (input, _) = char('\n')(input)?;
        Ok((input, text))
    }

    fn take_until_bracket(input: Span) -> IResult<Span> {
        take_while(|c| c != '[')(input)
    }

    // shortest match to avoid overeating
    // TODO(tkat0): refactor
    let ret = vec![
        peek(take_until_tag)(input),
        peek(take_until_bracket)(input),
        peek(take_until_newline)(input),
    ];

    let ret = ret
        .iter()
        .filter(|r| r.is_ok())
        .filter_map(|x| x.as_ref().ok())
        .min_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

    match ret {
        Some(&(input, consumed)) => {
            if consumed.is_empty() {
                return Err(Err::Error(ParseError::new(input, "".into())));
            }
            let input = input.slice(consumed.len()..);
            let text = Text {
                value: consumed.to_string(),
            };
            return Ok((input, text));
        }
        None => {
            return Err(Err::Error(ParseError::new(input, "".into())));
        }
    }
}

// [internal link]
fn internal_link(input: Span) -> IResult<InternalLink> {
    let (input, text) = bracket(input)?;
    Ok((input, InternalLink::new(text.fragment())))
}

// https://www.rust-lang.org/
fn external_link_plain(input: Span) -> IResult<ExternalLink> {
    map(url, |s| ExternalLink::new(None, &s))(input)
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
    let (input, text) = bracket(input)?;

    // [https://www.rust-lang.org/ Rust] or [https://www.rust-lang.org/]
    fn url_title(input: Span) -> IResult<ExternalLink> {
        let (input, _) = space0(input)?;
        let (input, url) = url(input)?;
        let (title, _) = space0(input)?;
        let title = if title.is_empty() { None } else { Some(title) };
        Ok((
            Span::new(""),
            ExternalLink::new(title.map(|span| *span.fragment()), &url),
        ))
    }

    // [Rust https://www.rust-lang.org/]
    // [Rust Rust Rust https://www.rust-lang.org/]
    // [Rust　Rust　Rust　https://www.rust-lang.org/]
    fn title_url(input: Span) -> IResult<ExternalLink> {
        let (input, title) = alt((
            take_until(" https://"),
            take_until(" http://"),
            take_until("　https://"),
            take_until("　http://"),
        ))(input)?;
        let title = if title.is_empty() { None } else { Some(title) };
        let (input, _) = space1(input)?;
        let (rest, url) = url(input)?;
        assert!(rest.is_empty());
        Ok((
            rest,
            ExternalLink::new(title.map(|span| *span.fragment()), &url),
        ))
    }

    let (rest, link) = alt((url_title, title_url))(text)?;
    assert!(rest.is_empty());
    Ok((input, link))
}

/// [http://cutedog.com https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
/// [https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
fn image(input: Span) -> IResult<Image> {
    let ext = ["svg", "jpg", "jpeg", "png", "gif"];
    let (input, text) = bracket(input)?;
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
    let (input, text) = bracket(input)?;

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
        Emphasis::new(text.fragment(), bold, italic, strikethrough),
    ))
}

// [[bold]]
fn bold(input: Span) -> IResult<Emphasis> {
    map(
        delimited(tag("[["), take_while(|c| c != ']'), tag("]]")),
        |s: Span| Emphasis::bold(*s),
    )(input)
}

/// [$ Tex here]
fn math() {}

/// `block_quate`
fn block_quate(input: Span) -> IResult<BlockQuate> {
    map(
        delimited(char('`'), take_while(|c| c != '`'), char('`')),
        |v: Span| BlockQuate::new(v.fragment()),
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
                file_name.fragment(),
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
        Ok((input, Table::new(*name, vec![], vec![vec![]])))
    }
}

/// >
fn quote() {}

/// $ hoge or % hoge
fn commandline(input: Span) -> IResult<BlockQuate> {
    let (input, prefix) = alt((tag("$ "), tag("% ")))(input)?;
    let prefix = prefix.fragment().to_string();
    let (input, text) = take_until_eol(input)?;
    Ok((input, BlockQuate::new(&format!("{}{}", prefix, text))))
}

/// ? hoge
fn helpfeel() {}

/// <tab>
/// <tab>1.
fn list(input: Span) -> IResult<Option<List>> {
    let (input, tabs) = opt(many1(char('\t')))(input)?;
    let (input, decimal) = opt(terminated(digit1, tag(". ")))(input)?;
    if let Some(tabs) = tabs {
        let kind = match &decimal {
            Some(_) => ListKind::Decimal,
            None => ListKind::Disc,
        };
        Ok((
            input,
            Some(List {
                level: tabs.len(),
                kind,
            }),
        ))
    } else {
        Ok((input, None))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest(input, expected,
        case("#tag", ("", HashTag::new("tag"))),
        case("#tag\n", ("\n", HashTag::new("tag"))),
        case("#tag ", (" ", HashTag::new("tag"))),
        case("#tag　", ("　", HashTag::new("tag"))),
        case("####tag", ("", HashTag::new("###tag"))),
        case("#[tag", ("", HashTag::new("[tag"))),
    )]
    fn hashtag_valid_test(input: &str, expected: (&str, HashTag)) {
        assert_eq!(
            hashtag(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    // #[rstest(input, case("#[tag]"), case("# tag"))]
    // fn hashtag_invalid_test(input: &str) {
    //     if let Ok(ok) = hashtag(Span::new(input)) {
    //         panic!("{:?}", ok)
    //     }
    // }

    #[rstest(input, expected,
        case("123abc", ("123abc", None)),
        case("\t\t123abc", ("123abc", Some(List::new(ListKind::Disc, 2)))),
        case("\t123. abc", ("abc", Some(List::new(ListKind::Decimal, 1)))),
    )]
    fn list_valid_test(input: &str, expected: (&str, Option<List>)) {
        assert_eq!(
            list(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("`code`", ("", BlockQuate::new("code"))),
        case("`code` test", (" test", BlockQuate::new("code"))),
    )]
    fn block_quate_valid_test(input: &str, expected: (&str, BlockQuate)) {
        assert_eq!(
            block_quate(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, case("123abc"), case("`123abc"))]
    fn block_quate_invalid_test(input: &str) {
        if let Ok(ok) = block_quate(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }

    #[rstest(input, expected,
        case("$ code   ", ("", BlockQuate::new("$ code   "))),
    )]
    fn commandline_valid_test(input: &str, expected: (&str, BlockQuate)) {
        assert_eq!(
            commandline(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("code:hello.rs\n     panic!()\n     panic!()\n", ("", CodeBlock::new("hello.rs", vec!["    panic!()", "    panic!()"]))),
    )]
    fn code_block_valid_test(input: &str, expected: (&str, CodeBlock)) {
        assert_eq!(
            code_block(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("table:table\n", ("", Table::new("table", vec![], vec![vec![]]))),
        case("table:table\n a\tb\tc\n d\te\tf\n", ("", Table::new("table", vec!["a".into(), "b".into(), "c".into()], vec![vec!["d".into(), "e".into(), "f".into()]]))),
        // case("table:table\n a\tb\tc\n", ("", Table::new("table", vec!["a".into(), "b".into(), "c".into()], vec![vec![]]))),
    )]
    fn table_valid_test(input: &str, expected: (&str, Table)) {
        assert_eq!(
            table(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
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
            image(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[* text]", ("", Emphasis::bold_level("text", 1))),
        case("[***** text]", ("", Emphasis::bold_level("text", 5))),
        case("[/ text]", ("", Emphasis::italic("text"))),
        case("[*/*-* text]", ("", Emphasis::new("text", 3, 1, 1))),
    )]
    fn emphasis_valid_test(input: &str, expected: (&str, Emphasis)) {
        assert_eq!(
            emphasis(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[[text]]", ("", Emphasis::bold_level("text", 1))),
    )]
    fn bold_valid_test(input: &str, expected: (&str, Emphasis)) {
        assert_eq!(
            bold(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case(" #tag", ("#tag", Text::new(" "))),
        case(" #tag[", ("#tag[", Text::new(" "))),
        case(" [#tag", ("[#tag", Text::new(" "))),
        case(" [ #tag", ("[ #tag", Text::new(" "))),
        case(" [url]", ("[url]", Text::new(" "))),
        case(" \n", ("\n", Text::new(" "))),
        case("abc#tag", ("", Text::new("abc#tag"))),
        case("abc #tag", ("#tag", Text::new("abc "))),
        case("あいう", ("", Text::new("あいう"))),
        case("[", ("", Text::new("["))),
    )]
    fn text_valid_test(input: &str, expected: (&str, Text)) {
        assert_eq!(
            text(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, case(""), case("#tag"))]
    fn text_invalid_test(input: &str) {
        if let Ok(ok) = text(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }

    #[rstest(input, expected,
        case("[title]", ("", InternalLink::new("title"))),
    )]
    fn internal_link_valid_test(input: &str, expected: (&str, InternalLink)) {
        assert_eq!(
            internal_link(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("https://www.rust-lang.org/ abc", (" abc", ExternalLink::new(None, "https://www.rust-lang.org/"))),
    )]
    fn external_link_plain_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link_plain(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[/help-jp/Scrapbox]", ("", ExternalLink::new(None, "https://scrapbox.io/help-jp/Scrapbox"))),
    )]
    fn external_link_other_project_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link_other_project(Span::new(input))
                .map(|(input, ret)| (*input.fragment(), ret)),
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
    )]
    fn external_link_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("abc #tag ", ("#tag ", Some(Expr::new(ExprKind::Text(Text::new("abc ")))))),
        case("[title]abc", ("abc", Some(Expr::new(ExprKind::InternalLink(InternalLink::new("title")))))),
        case("[", ("", Some(Expr::new(ExprKind::Text(Text::new("[")))))),
    )]
    fn expr_valid_test(input: &str, expected: (&str, Option<Expr>)) {
        assert_eq!(
            expr(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, case(""), case("\n"))]
    fn expr_invalid_test(input: &str) {
        if let Ok(ok) = text(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }

    #[rstest(input, expected,
        case(" ", ("", Line::new( LineKind::Normal, vec![Expr::new(ExprKind::Text(Text::new(" ")))]))),
        case("#tag #tag [internal link]\n", ("\n", Line::new(
            LineKind::Normal,
            vec![
                Expr::new(ExprKind::HashTag(HashTag::new("tag"))),
                Expr::new(ExprKind::Text(Text::new(" "))),
                Expr::new(ExprKind::HashTag(HashTag::new("tag"))),
                Expr::new(ExprKind::Text(Text::new(" "))),
                Expr::new(ExprKind::InternalLink(InternalLink::new("internal link")))
                ]
            ))),
    )]
    fn line_valid_test(input: &str, expected: (&str, Line)) {
        assert_eq!(
            line(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }

    #[rstest(input, case(""))]
    fn line_invalid_test(input: &str) {
        if let Ok(ok) = line(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }

    #[rstest(input, expected,
        case(indoc! {"
            abc
            #efg [internal link][https://www.rust-lang.org/]
        "}, ("", Page {
            lines: vec![
                Line::new(
                    LineKind::Normal,
                    vec![Expr::new(ExprKind::Text(Text {
                        value: "abc".into(),
                    }))],
                ),
                Line::new(
                    LineKind::Normal,
                    vec![
                        Expr::new(ExprKind::HashTag(HashTag {
                            value: "efg".into(),
                        })),
                        Expr::new(ExprKind::Text(Text { value: " ".into() })),
                        Expr::new(ExprKind::InternalLink(InternalLink::new("internal link"))),
                        Expr::new(ExprKind::ExternalLink(ExternalLink::new(
                            None,
                            "https://www.rust-lang.org/",
                        ))),
                    ],
                ),
                Line::new(LineKind::Normal, vec![]),
            ],
        }))
    )]
    fn page_valid_test(input: &str, expected: (&str, Page)) {
        assert_eq!(
            page(Span::new(input)).map(|(input, ret)| (*input.fragment(), ret)),
            Ok(expected)
        );
    }
}
