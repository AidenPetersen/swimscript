use nom::branch::alt;
use nom::bytes::complete::{is_not};
use nom::character::complete::{char, multispace0, space0, u32};
use nom::combinator::{map};
use nom::IResult;
use nom::multi::{many0};
use nom::sequence::preceded;
use nom::sequence::terminated;

#[derive(Debug, PartialEq, Clone)]
pub enum SetComponent {
    Subset(Box<SetComponent>),
    Label(String),
    Reps(u32),
    Distance(u32),
    Attributes(Vec<String>),
}

pub fn parse_label(i: &str) -> IResult<&str, SetComponent> {
    map(
        preceded(multispace0,
                 terminated(
                     is_not(":\n\r"),
                     char(':'),
                 )), |s: &str| SetComponent::Label(s.trim().to_string()))(i)
}


pub fn parse_reps(i: &str) -> IResult<&str, SetComponent> {
    map(
        preceded(space0,
                 terminated(
                     u32,
                     preceded(space0, char('*')),
                 )), |n| SetComponent::Reps(n),
    )(i)
}

pub fn parse_distance(i: &str) -> IResult<&str, SetComponent> {
    map(
        preceded(space0,
                 terminated(
                     u32, space0,
                 ),
        ), |n| SetComponent::Distance(n),
    )(i)
}


pub fn parser(i: &str) -> IResult<&str, Vec<SetComponent>> {
    many0(
        alt((
            parse_label,
            parse_reps,
            parse_distance,
        ))
    )(i)
}