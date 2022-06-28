use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    sequence::delimited,
};

use super::*;

// [abc]
pub fn bracket(input: Span) -> IResult<Span> {
    delimited(char('['), take_while(|c| c != ']'), char(']'))(input)
}

pub fn url(input: Span) -> IResult<String> {
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

pub fn space0(input: Span) -> IResult<Span> {
    take_while(is_space)(input)
}

pub fn space1(input: Span) -> IResult<Span> {
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
        case("[]", ("", "")),
        case("[abc]def", ("def", "abc")),
        case("[ab]c]def", ("c]def", "ab")),
    )]
    fn bracket_valid_test(input: &str, expected: (&str, &str)) {
        assert_eq!(
            bracket(Span::new(input)).map(|(input, ret)| (*input, *ret)),
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
}
