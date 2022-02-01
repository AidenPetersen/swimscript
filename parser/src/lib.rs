use nom::error::VerboseError;
use nom::IResult;

pub mod parser;
#[cfg(test)]
mod tests;

pub fn to_json(data: &str) -> Result<String, nom::Err<VerboseError<&str>>> {
    let parsed = parser::parse(data);

    // Should propagate parser error through, serde shouldn't error.
    match parsed {
        IResult::Ok(result) => Ok(serde_json::to_string(&result.1).unwrap()),
        IResult::Err(e) => Err(e)
    }
}


pub fn to_ron(data: &str) -> Result<String, nom::Err<VerboseError<&str>>> {
    let parsed = parser::parse(data);
    match parsed {
        IResult::Ok(result) => Ok(ron::to_string(&result.1).unwrap()),
        IResult::Err(e) => Err(e)
    }
}