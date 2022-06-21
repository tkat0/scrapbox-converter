use std::convert::identity;

use nom::character::complete::{char, line_ending};
use nom::combinator::{eof, opt, peek};
use nom::error::{Error, ParseError, VerboseError};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while, take_while_m_n},
    character::{
        complete::{anychar, newline, none_of, space1},
        is_newline,
    },
    combinator::{map, map_res},
    multi::{fold_many0, many0},
    sequence::{delimited, preceded, terminated, tuple},
    Parser,
};
use nom::{error_position, IResult};

use crate::ast::*;

pub type Result<I, O, E = VerboseError<I>> = IResult<I, O, E>;

fn sp(input: &str) -> Result<&str, &str> {
    let chars = "\t[#\n";

    take_while(move |c| chars.contains(c))(input)
}

// fn page<'a, E: ParseError<&'a str>>(input: &'a str) -> Result<&'a str, Page, E> {
fn page(input: &str) -> Result<&str, Page> {
    let (input, lines) = many0(line)(input)?;

    Ok((input, Page { lines }))
}

fn line(input: &str) -> Result<&str, Line> {
    map(many0(syntax), |c| Line {
        items: c.into_iter().filter_map(identity).collect(),
    })(input)
}

fn syntax(input: &str) -> Result<&str, Option<Syntax>> {
    dbg!(input);

    // let (input, _) = opt(char('\n'))(input)?;
    // dbg!(input);

    // if input.is_empty() {
    //     return Ok(("", None));
    // }

    map(
        alt((
            map(hashtag, |s| Syntax {
                kind: SyntaxKind::HashTag(s),
            }),
            map(bracketing, |s| Syntax {
                kind: SyntaxKind::Bracket(s),
            }),
            map(text, |s| Syntax {
                kind: SyntaxKind::Text(s),
            }),
        )),
        Some,
    )(input)
}

/// #tag
fn hashtag(input: &str) -> Result<&str, HashTag> {
    let terminators = vec![" ", "　", "\n"];
    map(
        preceded(
            tag("#"),
            take_while(move |c: char| !terminators.contains(&c.to_string().as_str())),
            // alt((
            //     take_until(" "),
            //     take_until("　"), // zenkaku space
            //     take_until("\n"),
            //     take_while(|_| true), // eol
            // )),
        ),
        |s: &str| HashTag {
            value: s.to_string(),
        },
    )(input)
}

// only consume \n
/// "xxx #tag" -> "xxx"
fn text(input: &str) -> Result<&str, Text> {
    if input.is_empty() {
        // return Err(nom::Err::Error(error_position!(
        //     input,
        //     nom::error::ErrorKind::Alpha
        // )));
        return Err(nom::Err::Error(error_position!(
            input,
            nom::error::ErrorKind::Alpha
        )));
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
        let (input, _) = char('\n')(input)?;
        Ok((input, text))
    }

    let chars = "[#\n";
    map(
        alt((
            take_until_tag,
            take_while(|c| c != '['),
            take_until_newline,
            // take_while(move |c| !chars.contains(c)),
        )),
        // take_while(|c| c != '[' || c != '#' || c != '\n'),
        |s: &str| Text {
            value: s.to_string(),
        },
    )(input)
}

fn anystring(input: &str) -> Result<&str, String> {
    fold_many0(anychar, String::new, |mut string, c| {
        string.push(c);
        string
    })(input)
}

/// []
fn bracketing(input: &str) -> Result<&str, Bracket> {
    let (input, _) = peek(delimited(char('['), take_while(|c| c != ']'), char(']')))(input)?;
    map(
        alt((
            map(external_link, |c| BracketKind::ExternalLink(c)),
            map(internal_link, |c| BracketKind::InternalLink(c)),
        )),
        |kind| Bracket { kind },
    )(input)
}

// [internal link]
// fn internal_link<'a, E: ParseError<&'a str>>(input: &'a str) -> Result<&'a str, InternalLink, E> {
// fn internal_link<'a>(input: &'a str) -> Result<&'a str, InternalLink, VerboseError<&'a str>> {
fn internal_link(input: &str) -> Result<&str, InternalLink> {
    let (input, text) = delimited(char('['), take_while(|c| c != ']'), char(']'))(input)?;
    Ok((input, InternalLink::new(text)))
}

/// https://google.com or [https://google.com] or [https://google.com Google] or [Google https://google.com]
fn external_link(input: &str) -> Result<&str, ExternalLink> {
    // [https://google.com]
    fn url(input: &str) -> Result<&str, ExternalLink> {
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until("]")(input)?;
        // let (input, _) = char(']')(input)?;
        Ok((
            input,
            ExternalLink::new(None, &format!("{}{}", protocol, url)),
        ))
    }

    // [https://google.com Google]
    fn url_title(input: &str) -> Result<&str, ExternalLink> {
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until(" ")(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, title) = take_until("]")(input)?;
        // let (input, _) = char(']')(input)?;
        Ok((
            input,
            ExternalLink::new(Some(title), &format!("{}{}", protocol, url)),
        ))
    }

    // [Google https://google.com]
    fn title_url(input: &str) -> Result<&str, ExternalLink> {
        let (input, title) = take_until(" ")(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until("]")(input)?;
        // let (input, _) = char(']')(input)?;
        Ok((
            input,
            ExternalLink::new(Some(title), &format!("{}{}", protocol, url)),
        ))
    }

    delimited(char('['), alt((url_title, title_url, url)), char(']'))(input)
}

/// [http://cutedog.com https://i.gyazo.com/da78df293f9e83a74b5402411e2f2e01.png]
fn image() {}

/// [/icons/todo.icon]
fn icon() {}

// [*-/** decoration]
fn decoration() {}

/// [[Bold]] or [* Bold] or [*** Bold]
fn bold() {}

/// [/ italic]
fn italic() {}

/// [- strikethrough]
fn strikethrough() {}

/// [$ Tex here]
fn math() {}

/// `block_quate`
fn block_quate() {}

/// code:filename.extension
fn code_block() {}

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
fn bullet_points() {}

mod test {
    use nom::error::{Error, ErrorKind};

    use super::*;

    #[test]
    fn hashtag_test() {
        assert_eq!(hashtag("#tag"), Ok(("", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag\n"), Ok(("\n", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag "), Ok((" ", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag　"), Ok(("　", HashTag::new("tag"))));
    }

    #[test]
    fn text_test() {
        assert!(text("[* bold]").is_err());
        assert!(text("").is_err());
        // assert_eq!(text("[* bold]"), Ok(("", Text::new("[* bold]"))));
        assert_eq!(text("#tag"), Ok(("", Text::new("#tag")))); // TODO(tkat0): consider this spec.
        assert_eq!(text("abc#tag"), Ok(("", Text::new("abc#tag"))));
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
    fn external_link_test() {
        assert_eq!(
            external_link("[https://google.com]"),
            Ok(("", ExternalLink::new(None, "https://google.com")))
        );
        assert_eq!(
            external_link("[Google https://google.com]"),
            Ok(("", ExternalLink::new(Some("Google"), "https://google.com")))
        );
        assert_eq!(
            external_link("[https://google.com Google]"),
            Ok(("", ExternalLink::new(Some("Google"), "https://google.com")))
        );
        // assert_eq!(
        //     external_link("https://google.com"),
        //     Ok(("", ExternalLink::new(None, "https://google.com]")))
        // );
    }

    #[test]
    fn bracketing_test() {
        assert_eq!(
            bracketing("[title]"),
            Ok((
                "",
                Bracket {
                    kind: BracketKind::InternalLink(InternalLink::new("title"))
                }
            ))
        );
        assert_eq!(
            bracketing("[https://google.com]"),
            Ok((
                "",
                Bracket {
                    kind: BracketKind::ExternalLink(ExternalLink::new(None, "https://google.com"))
                }
            ))
        );
    }

    #[test]
    fn syntax_test() {
        assert_eq!(syntax(""), Ok(("", None)));
        assert_eq!(
            syntax("#tag"),
            Ok((
                "",
                Some(Syntax {
                    kind: SyntaxKind::HashTag(HashTag::new("tag"))
                })
            ))
        );
        assert_eq!(
            syntax("[title]"),
            Ok((
                "",
                Some(Syntax {
                    kind: SyntaxKind::Bracket(Bracket {
                        kind: BracketKind::InternalLink(InternalLink::new("title"))
                    })
                })
            ))
        );
        // assert_eq!(syntax("[* bold]"), Ok(("[* bold]", Text::new(""))));
        // assert_eq!(syntax("abc"), Ok(("", Text::new("abc"))));
        // assert_eq!(syntax("あいう"), Ok(("", Text::new("あいう"))));
    }

    #[test]
    fn line_test() {
        many0(syntax)("x").unwrap();

        // assert_eq!(
        //     line(" "),
        //     Ok((
        //         "",
        //         Line {
        //             items: vec![Syntax {
        //                 kind: SyntaxKind::Text(Text::new(" "))
        //             },],
        //         }
        //     ))
        // );
        // assert_eq!(
        //     line("#tag #tag\n"),
        //     Ok((
        //         "",
        //         Line {
        //             items: vec![
        //                 Syntax {
        //                     kind: SyntaxKind::HashTag(HashTag::new("tag"))
        //                 },
        //                 Syntax {
        //                     kind: SyntaxKind::Text(Text::new(" "))
        //                 },
        //                 Syntax {
        //                     kind: SyntaxKind::HashTag(HashTag::new("tag"))
        //                 },
        //             ],
        //         }
        //     ))
        // );
    }

    #[test]
    fn page_test() {
        let actual = page("abc\n#efg [internal link]\n");
        let expected = Page {
            lines: vec![
                Line {
                    items: vec![Syntax {
                        kind: SyntaxKind::Text(Text {
                            value: "abc".to_string(),
                        }),
                    }],
                },
                Line {
                    items: vec![
                        Syntax {
                            kind: SyntaxKind::HashTag(HashTag {
                                value: "efg".to_string(),
                            }),
                        },
                        Syntax {
                            kind: SyntaxKind::Text(Text {
                                value: " ".to_string(),
                            }),
                        },
                        Syntax {
                            kind: SyntaxKind::Bracket(Bracket {
                                kind: BracketKind::InternalLink(InternalLink::new("internal link")),
                            }),
                        },
                    ],
                },
            ],
        };
        assert_eq!(actual, Ok(("", expected)))
    }
}
