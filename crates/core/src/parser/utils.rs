use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while, take_while1},
    character::complete::char,
    combinator::{map, opt, peek},
    sequence::{delimited, preceded},
    Err, Slice,
};

use crate::ast::*;

use super::*;

pub fn take_until_eol<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
    alt((take_until("\n"), take(input.chars().count())))(input)
}

// [abc]
pub fn brackets<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
    delimited(char('['), take_while(|c| c != ']'), char(']'))(input)
}

// (abc)
pub fn parentheses<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
    delimited(char('('), take_while(|c| c != ')'), char(')'))(input)
}

pub fn url<X: Clone>(input: Span<X>) -> IResult<String, X> {
    let (url, protocol) = alt((tag("https://"), tag("http://")))(input)?;

    fn is_token(c: char) -> bool {
        match c as u8 {
            33..=126 => true,
            _ => false,
        }
    }

    let (rest, url) = take_while(|c| is_token(c))(url)?;

    Ok((rest, format!("{}{}", protocol, url)))
}

// https://www.rust-lang.org/
pub fn external_link_plain<X: Clone>(input: Span<X>) -> IResult<ExternalLink, X> {
    map(url, |s| ExternalLink::new(None, &s))(input)
}

/// #tag
pub fn hashtag<X: Clone>(input: Span<X>) -> IResult<HashTag, X> {
    let terminators = vec![" ", "　", "\n"];

    // TODO(tkat0): "#[tag]" -> Error
    //  it should be handled with text + internal link

    map(
        preceded(
            tag("#"),
            take_while(move |c: char| !terminators.contains(&c.to_string().as_str())),
        ),
        |s: Span<X>| HashTag::new(*s),
    )(input)
}

pub fn text<X: Clone + Copy>(input: Span<X>) -> IResult<Text, X> {
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
    fn take_until_tag<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
        // " #tag" -> ("#tag", " ")
        // allow "abc#tag"
        let (input, _) = peek(take_until(" #"))(input)?;
        take_until("#")(input)
    }

    fn take_until_bracket<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
        take_while(|c| c != '[')(input)
    }

    // shortest match to avoid overeating
    // TODO(tkat0): refactor
    let ret = vec![
        peek(take_until_tag)(input),
        peek(take_until_bracket)(input),
        peek(take_until_eol)(input),
        peek(take_until("`"))(input),
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

/// `block_quate`
pub fn block_quate<X: Clone>(input: Span<X>) -> IResult<BlockQuate, X> {
    map(
        delimited(char('`'), take_while(|c| c != '`'), char('`')),
        |s: Span<X>| BlockQuate::new(*s),
    )(input)
}

pub fn space0<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
    take_while(is_space)(input)
}

pub fn space1<X: Clone>(input: Span<X>) -> IResult<Span<X>, X> {
    take_while1(is_space)(input)
}

pub fn is_space(c: char) -> bool {
    c == ' ' || c == '　'
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
        case("", ("", "")),
        case("abc\ndef", ("\ndef", "abc")),
        case("abcdef", ("", "abcdef")),
    )]
    fn take_until_eol_valid_test(input: &str, expected: (&str, &str)) {
        assert_eq!(
            take_until_eol(Span::new(input)).map(|(input, ret)| (*input, *ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("[]", ("", "")),
        case("[abc]def", ("def", "abc")),
        case("[ab]c]def", ("c]def", "ab")),
    )]
    fn brackets_valid_test(input: &str, expected: (&str, &str)) {
        assert_eq!(
            brackets(Span::new(input)).map(|(input, ret)| (*input, *ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("http://www.rust-lang.org", ("", "http://www.rust-lang.org".into())),
        case("https://www.rust-lang.org", ("", "https://www.rust-lang.org".into())),
        case("https://www.rust-lang.org abc", (" abc", "https://www.rust-lang.org".into())),
    )]
    fn url_valid_test(input: &str, expected: (&str, String)) {
        assert_eq!(
            url(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, expected,
        case("https://www.rust-lang.org/ abc", (" abc", ExternalLink::new(None, "https://www.rust-lang.org/"))),
    )]
    fn external_link_plain_valid_test(input: &str, expected: (&str, ExternalLink)) {
        assert_eq!(
            external_link_plain(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

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
            hashtag(Span::new(input)).map(|(input, ret)| (*input, ret)),
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
        case(" #tag", ("#tag", Text::new(" "))),
        case(" #tag[", ("#tag[", Text::new(" "))),
        case(" [#tag", ("[#tag", Text::new(" "))),
        case(" [ #tag", ("[ #tag", Text::new(" "))),
        case(" [url]", ("[url]", Text::new(" "))),
        case(" \n", ("\n", Text::new(" "))),
        case("abc`aaa`", ("`aaa`", Text::new("abc"))),
        case("abc#tag", ("", Text::new("abc#tag"))),
        case("abc #tag", ("#tag", Text::new("abc "))),
        case("あいう", ("", Text::new("あいう"))),
        case("[", ("", Text::new("["))),
    )]
    fn text_valid_test(input: &str, expected: (&str, Text)) {
        assert_eq!(
            text(Span::new(input)).map(|(input, ret)| (*input, ret)),
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
        case("`code`", ("", BlockQuate::new("code"))),
        case("`code` test", (" test", BlockQuate::new("code"))),
    )]
    fn block_quate_valid_test(input: &str, expected: (&str, BlockQuate)) {
        assert_eq!(
            block_quate(Span::new(input)).map(|(input, ret)| (*input, ret)),
            Ok(expected)
        );
    }

    #[rstest(input, case("123abc"), case("`123abc"))]
    fn block_quate_invalid_test(input: &str) {
        if let Ok(ok) = block_quate(Span::new(input)) {
            panic!("{:?}", ok)
        }
    }
}
