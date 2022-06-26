use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    sequence::delimited,
};

use super::*;

// [abc]
pub fn bracket(input: &str) -> Result<&str, &str> {
    delimited(char('['), take_while(|c| c != ']'), char(']'))(input)
}

pub fn url(input: &str) -> Result<&str, String> {
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

pub fn space0(input: &str) -> Result<&str, &str> {
    take_while(is_space)(input)
}

pub fn space1(input: &str) -> Result<&str, &str> {
    take_while1(is_space)(input)
}

pub fn is_space(c: char) -> bool {
    c == ' ' || c == '　'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bracket_test() {
        assert_eq!(bracket("[]"), Ok(("", "")));
        assert_eq!(bracket("[abc]def"), Ok(("def", "abc")));
        assert_eq!(bracket("[ab]c]def"), Ok(("c]def", "ab")));
    }

    #[test]
    fn url_test() {
        assert_eq!(
            url("https://www.rust-lang.org"),
            Ok(("", "https://www.rust-lang.org".into()))
        );
        assert_eq!(
            url("https://www.rust-lang.org abc"),
            Ok((" abc", "https://www.rust-lang.org".into()))
        );
    }
}
