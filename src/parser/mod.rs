use std::convert::identity;

use nom::character::complete::{char, digit1, space0, space1};
use nom::combinator::{opt, peek};
use nom::error::{ParseError, VerboseError};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded},
    Err,
};

use crate::ast::*;

pub type Result<I, O, E = VerboseError<I>> = IResult<I, O, E>;

pub fn page(input: &str) -> Result<&str, Page> {
    let (input, lines) = many0(line)(input)?;

    Ok((input, Page { lines }))
}

fn line(input: &str) -> Result<&str, Line> {
    if input.is_empty() {
        return Err(Err::Error(VerboseError::from_char(input, ' ')));
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

fn expr(input: &str) -> Result<&str, Option<Expr>> {
    map(
        alt((
            map(hashtag, |s| Expr::new(ExprKind::HashTag(s))),
            map(block_quate, |s| Expr::new(ExprKind::BlockQuate(s))),
            map(code_block, |s| Expr::new(ExprKind::CodeBlock(s))),
            map(emphasis, |c| Expr::new(ExprKind::Emphasis(c))),
            map(external_link, |c| Expr::new(ExprKind::ExternalLink(c))),
            map(internal_link, |c| Expr::new(ExprKind::InternalLink(c))),
            map(external_link_plain, |s| {
                Expr::new(ExprKind::ExternalLink(s))
            }),
            map(text, |s| Expr::new(ExprKind::Text(s))),
        )),
        Some,
    )(input)
}

/// #tag
fn hashtag(input: &str) -> Result<&str, HashTag> {
    let terminators = vec![" ", "　", "\n"];

    // TODO(tkat0): "#[tag]" -> Error
    //  it should be handled with text + internal link

    map(
        preceded(
            tag("#"),
            take_while(move |c: char| !terminators.contains(&c.to_string().as_str())),
        ),
        |s: &str| HashTag {
            value: s.to_string(),
        },
    )(input)
}

fn text(input: &str) -> Result<&str, Text> {
    if input.is_empty() {
        return Err(Err::Error(VerboseError::from_char(input, 'x')));
    }

    if input.starts_with("#") {
        return Err(Err::Error(VerboseError::from_char(input, ' ')));
    }

    // "abc #tag" -> ("#tag", "abc ")
    fn take_until_tag(input: &str) -> Result<&str, &str> {
        // " #tag" -> ("#tag", " ")
        // allow "abc#tag"
        let (input, _) = peek(take_until(" #"))(input)?;
        take_until("#")(input)
    }

    // "abc \n" -> ("", "abc ")
    fn take_until_newline(input: &str) -> Result<&str, &str> {
        let (input, text) = take_until("\n")(input)?;
        // let (input, _) = char('\n')(input)?;
        Ok((input, text))
    }

    fn take_until_bracket(input: &str) -> Result<&str, &str> {
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
                return Err(Err::Error(VerboseError::from_char(input, ' ')));
            }
            let input = input.split_at(consumed.len()).1;
            let text = Text {
                value: consumed.to_string(),
            };
            return Ok((input, text));
        }
        None => {
            return Err(Err::Error(VerboseError::from_char(input, ' ')));
        }
    }
}

// [internal link]
fn internal_link(input: &str) -> Result<&str, InternalLink> {
    let (input, text) = delimited(char('['), take_while(|c| c != ']'), char(']'))(input)?;
    Ok((input, InternalLink::new(text)))
}

// https://www.rust-lang.org/
fn external_link_plain(input: &str) -> Result<&str, ExternalLink> {
    let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
    let (input, url) = take_until(" ")(input)?; // TODO(tkat0): zenkaku
    Ok((
        input,
        ExternalLink::new(None, &format!("{}{}", protocol, url)),
    ))
}

/// [https://www.rust-lang.org/] or [https://www.rust-lang.org/ Rust] or [Rust https://www.rust-lang.org/]
fn external_link(input: &str) -> Result<&str, ExternalLink> {
    let (input, text) = delimited(char('['), take_while(|c| c != ']'), char(']'))(input)?;

    dbg!(text);

    // [https://www.rust-lang.org/]
    fn url(input: &str) -> Result<&str, ExternalLink> {
        let (input, _) = space0(input)?; // TODO(tkat0): zenkaku
        let (url, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        Ok((
            "",
            ExternalLink::new(None, &format!("{}{}", protocol, url.trim())),
        ))
    }

    // [https://www.rust-lang.org/ Rust]
    fn url_title(input: &str) -> Result<&str, ExternalLink> {
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until(" ")(input)?;
        let (title, _) = space1(input)?; // TODO(tkat0): zenkaku
        let title = if title.is_empty() { None } else { Some(title) };
        Ok((
            "",
            ExternalLink::new(title, &format!("{}{}", protocol, url)),
        ))
    }

    // [Rust https://www.rust-lang.org/]
    fn title_url(input: &str) -> Result<&str, ExternalLink> {
        let (input, title) = take_until(" ")(input)?;
        let title = if title.is_empty() { None } else { Some(title) };
        let (input, _) = space1(input)?; // TODO(tkat0): zenkaku
        let (url, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        Ok((
            "",
            ExternalLink::new(title, &format!("{}{}", protocol, url.trim())),
        ))
    }

    let (rest, link) = alt((url_title, title_url, url))(text)?;
    dbg!(rest);
    dbg!(&link);
    assert!(rest.is_empty());
    Ok((input, link))
}

/// [http://cutedog.com https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
fn image() {}

/// [/icons/todo.icon]
fn icon() {}

/// [*-/** emphasis]
/// [[Bold]] or [* Bold] or [*** Bold]
/// [/ italic]
/// [- strikethrough]
fn emphasis(input: &str) -> Result<&str, Emphasis> {
    let (input, text) = delimited(char('['), take_while(|c| c != ']'), char(']'))(input)?;

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

    Ok((input, Emphasis::new(text, bold, italic, strikethrough)))
}

/// [$ Tex here]
fn math() {}

/// `block_quate`
fn block_quate(input: &str) -> Result<&str, BlockQuate> {
    map(
        delimited(char('`'), take_while(|c| c != '`'), char('`')),
        |v| BlockQuate::new(v),
    )(input)
}

/// code:filename.extension
/// TODO(tkat0): List + CodeBlock is not supported yet.
fn code_block(input: &str) -> Result<&str, CodeBlock> {
    let (input, _) = tag("code:")(input)?;
    let (input, file_name) = take_until("\n")(input)?;
    let (input, _) = char('\n')(input)?;
    map(
        many0(delimited(char('\t'), take_while(|c| c != '\n'), char('\n'))),
        |codes| CodeBlock::new(file_name, codes),
    )(input)
}

/// table:name
/// a<tab>
fn table() {}

/// >
fn quote() {}

/// $ hoge or % hoge
fn commandline() {}

/// ? hoge
fn helpfeel() {}

/// <tab>
/// <tab>1.
fn list(input: &str) -> Result<&str, Option<List>> {
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

mod test {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn hashtag_test() {
        assert_eq!(hashtag("#tag"), Ok(("", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag\n"), Ok(("\n", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag "), Ok((" ", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag　"), Ok(("　", HashTag::new("tag"))));
        assert_eq!(hashtag("####tag"), Ok(("", HashTag::new("###tag"))));
        assert_eq!(hashtag("#[tag"), Ok(("", HashTag::new("[tag"))));
        // assert!(hashtag("#[tag]").is_err());
        // assert!(hashtag("# tag").is_err());
    }

    #[test]
    fn list_test() {
        assert_eq!(list("123abc"), Ok(("123abc", None)));
        assert_eq!(
            list("\t\t123abc"),
            Ok(("123abc", Some(List::new(ListKind::Disc, 2))))
        );
        assert_eq!(
            list("\t123. abc"),
            Ok(("abc", Some(List::new(ListKind::Decimal, 1))))
        );
    }

    #[test]
    fn block_quate_test() {
        assert!(block_quate("123abc").is_err());
        assert!(block_quate("`123abc").is_err());
        assert_eq!(block_quate("`code`"), Ok(("", BlockQuate::new("code"))));
        assert_eq!(
            block_quate("`code` test"),
            Ok((" test", BlockQuate::new("code")))
        );
    }

    #[test]
    fn code_block_test() {
        assert_eq!(
            code_block("code:hello.rs\n\t    panic!()\n\t    panic!()\n"),
            Ok((
                "",
                CodeBlock::new("hello.rs", vec!["    panic!()", "    panic!()"])
            ))
        );
    }

    #[test]
    fn emphasis_test() {
        assert_eq!(
            emphasis("[* text]"),
            Ok(("", Emphasis::bold_level("text", 1)))
        );
        assert_eq!(
            emphasis("[***** text]"),
            Ok(("", Emphasis::bold_level("text", 5)))
        );
        assert_eq!(emphasis("[/ text]"), Ok(("", Emphasis::italic("text"))));
        assert_eq!(
            emphasis("[- text]"),
            Ok(("", Emphasis::strikethrough("text")))
        );
        assert_eq!(
            emphasis("[*/*-* text]"),
            Ok(("", Emphasis::new("text", 3, 1, 1)))
        );
    }

    #[test]
    fn text_test() {
        assert!(text("").is_err());
        assert!(text("[* bold]").is_err());
        assert!(text("#tag").is_err());
        assert_eq!(text(" #tag"), Ok(("#tag", Text::new(" "))));
        assert_eq!(text(" [url]"), Ok(("[url]", Text::new(" "))));
        assert_eq!(text(" #tag["), Ok(("#tag[", Text::new(" "))));
        assert_eq!(text(" [#tag"), Ok(("[#tag", Text::new(" "))));
        assert_eq!(text(" [ #tag"), Ok(("[ #tag", Text::new(" "))));
        assert_eq!(text(" \n"), Ok(("\n", Text::new(" "))));
        assert_eq!(text("abc#tag"), Ok(("", Text::new("abc#tag"))));
        assert_eq!(text("abc #tag"), Ok(("#tag", Text::new("abc "))));
        assert_eq!(text(" #tag"), Ok(("#tag", Text::new(" "))));
        assert_eq!(text("abc"), Ok(("", Text::new("abc"))));
        assert_eq!(text("あいう"), Ok(("", Text::new("あいう"))));
    }

    #[test]
    fn internal_link_test() {
        assert_eq!(
            internal_link("[title]"),
            Ok(("", InternalLink::new("title")))
        );
    }

    #[test]
    fn external_link_plain_test() {
        assert_eq!(
            external_link_plain("https://www.rust-lang.org/ abc"),
            Ok((
                " abc",
                ExternalLink::new(None, "https://www.rust-lang.org/")
            ))
        );
    }

    #[test]
    fn external_link_test() {
        assert_eq!(
            external_link("[https://www.rust-lang.org/]"),
            Ok(("", ExternalLink::new(None, "https://www.rust-lang.org/")))
        );
        assert_eq!(
            external_link("[  https://www.rust-lang.org/]"),
            Ok(("", ExternalLink::new(None, "https://www.rust-lang.org/")))
        );
        assert_eq!(
            external_link("[  https://www.rust-lang.org/  ]"),
            Ok(("", ExternalLink::new(None, "https://www.rust-lang.org/")))
        );
        assert_eq!(
            external_link("[Rust https://www.rust-lang.org/]"),
            Ok((
                "",
                ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/")
            ))
        );
        assert_eq!(
            external_link("[https://www.rust-lang.org/ Rust]"),
            Ok((
                "",
                ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/")
            ))
        );
        assert_eq!(
            external_link("[https://www.rust-lang.org/    Rust]"),
            Ok((
                "",
                ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/")
            ))
        );
        // assert!(external_link("[ https://www.rust-lang.org/ Rust ]").is_err());
        assert_eq!(
            external_link("[https://www.rust-lang.org/]\n[*-/ text]"),
            Ok((
                "\n[*-/ text]",
                ExternalLink::new(None, "https://www.rust-lang.org/")
            ))
        );
    }

    #[test]
    fn expr_test() {
        assert!(expr("").is_err());
        assert!(expr("\n").is_err());
        assert_eq!(
            expr("abc #tag "),
            Ok(("#tag ", Some(Expr::new(ExprKind::Text(Text::new("abc "))))))
        );
        assert_eq!(
            expr("#tag abc"),
            Ok((
                " abc",
                Some(Expr::new(ExprKind::HashTag(HashTag::new("tag"))))
            ))
        );
        assert_eq!(
            expr("[title]abc"),
            Ok((
                "abc",
                Some(Expr::new(ExprKind::InternalLink(InternalLink::new(
                    "title"
                ))))
            ))
        );
    }

    #[test]
    fn line_test() {
        assert!(line("").is_err());
        assert_eq!(
            line(" "),
            Ok((
                "",
                Line::new(
                    LineKind::Normal,
                    vec![Expr::new(ExprKind::Text(Text::new(" ")))]
                ),
            ))
        );
        assert_eq!(
            line("#tag #tag [internal link]\n"),
            Ok((
                "\n",
                Line::new(
                    LineKind::Normal,
                    vec![
                        Expr::new(ExprKind::HashTag(HashTag::new("tag"))),
                        Expr::new(ExprKind::Text(Text::new(" "))),
                        Expr::new(ExprKind::HashTag(HashTag::new("tag"))),
                        Expr::new(ExprKind::Text(Text::new(" "))),
                        Expr::new(ExprKind::InternalLink(InternalLink::new("internal link"))),
                    ]
                )
            ))
        );
    }

    #[test]
    fn page_test() {
        let actual = page("abc\n#efg [internal link][https://www.rust-lang.org/]\n");
        let expected = Page {
            lines: vec![
                Line::new(
                    LineKind::Normal,
                    vec![Expr::new(ExprKind::Text(Text {
                        value: "abc".to_string(),
                    }))],
                ),
                Line::new(
                    LineKind::Normal,
                    vec![
                        Expr::new(ExprKind::HashTag(HashTag {
                            value: "efg".to_string(),
                        })),
                        Expr::new(ExprKind::Text(Text {
                            value: " ".to_string(),
                        })),
                        Expr::new(ExprKind::InternalLink(InternalLink::new("internal link"))),
                        Expr::new(ExprKind::ExternalLink(ExternalLink::new(
                            None,
                            "https://www.rust-lang.org/",
                        ))),
                    ],
                ),
                Line::new(LineKind::Normal, vec![]),
            ],
        };
        assert_eq!(actual, Ok(("", expected)))
    }
}
