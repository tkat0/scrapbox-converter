use nom_locate::LocatedSpan;

pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    span: Span<'a>,
    message: Option<String>,
}

impl<'a> ParseError<'a> {
    pub fn new(span: Span<'a>, message: String) -> Self {
        Self {
            span,
            message: Some(message),
        }
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    #[inline]
    pub fn line(&self) -> u32 {
        self.span().location_line()
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.span().location_offset()
    }

    pub fn message(&self) -> String {
        if let Some(message) = &self.message {
            return format!(
                "{}:{}: parse error: {}",
                self.line(),
                self.offset(),
                message
            );
        }
        todo!()
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(input, format!("parse error: {:?}", kind))
    }

    fn append(input: Span, kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(input, format!("unexpected character: '{}'", c))
    }

    fn or(self, other: Self) -> Self {
        other
    }
}
