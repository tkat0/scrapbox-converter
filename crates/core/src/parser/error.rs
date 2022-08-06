use nom_locate::LocatedSpan;

pub type IResult<'a, O, X = ()> = nom::IResult<Span<'a, X>, O, ParseError<'a, X>>;

pub type Span<'a, X = ()> = LocatedSpan<&'a str, X>;

#[derive(Debug, PartialEq)]
pub struct ParseError<'a, X = ()> {
    span: Span<'a, X>,
    message: Option<String>,
}

impl<'a, X> ParseError<'a, X> {
    pub fn new(span: Span<'a, X>, message: String) -> Self {
        Self {
            span,
            message: Some(message),
        }
    }

    #[inline]
    pub fn span(&self) -> &Span<X> {
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

impl<'a, X> nom::error::ParseError<Span<'a, X>> for ParseError<'a, X> {
    fn from_error_kind(input: Span<'a, X>, kind: nom::error::ErrorKind) -> Self {
        Self::new(input, format!("parse error: {:?}", kind))
    }

    fn append(input: Span<X>, kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a, X>, c: char) -> Self {
        Self::new(input, format!("unexpected character: '{}'", c))
    }

    fn or(self, other: Self) -> Self {
        other
    }
}
