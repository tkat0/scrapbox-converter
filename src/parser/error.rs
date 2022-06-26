use nom::error::VerboseError;
use nom::IResult;

pub type Result<I, O, E = VerboseError<I>> = IResult<I, O, E>;
