use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{char, multispace0, space0, u32};
use nom::combinator::{map, opt};
use nom::error::{context, VerboseError};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::sequence::terminated;
use serde::{Serialize};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SetComponent {
    Label(String),
    Reps(u32),
    Distance(u32),
    Attributes(Vec<String>),
    Subset(Vec<Set>),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SetData {
    label: Option<String>,
    reps: u32,
    attributes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Set {
    Subset { subsets: Vec<Set>, data: SetData },
    Single { distance: u32, data: SetData },
    Text(String),
}

fn parse_label(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context(
            "label",
            terminated(is_not(":\n0123456789"), char(':')),
        ),
        |s: &str| SetComponent::Label(s.to_string()),
    )(i)
}

fn parse_reps(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context("reps",
                terminated(
                    preceded(
                        space0,
                        terminated(u32, preceded(space0, char('*')),
                        )), space0),
        ),
        |n: u32| SetComponent::Reps(n))(i)
}

fn parse_distance(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context("distance",
                preceded(space0, terminated(u32, space0)),
        ),
        |n: u32| SetComponent::Distance(n))(i)
}

fn parse_subset(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context("subset",
                preceded(
                    multispace0, preceded(
                        char('{'), terminated(
                            root_parser, char('}'),
                        )))),
        |x| SetComponent::Subset(x))(i)
}

fn parse_attributes(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context("attributes",
                preceded(
                    space0, separated_list1(
                        char(','), preceded(space0, is_not(",\n\r")))),
        ),
        |x: Vec<&str>| SetComponent::Attributes(x.iter().map(|y| y.trim().to_string()).collect()),
    )(i)
}

fn parse_section(i: &str) -> IResult<&str, Set, VerboseError<&str>> {
    let (i, e) = context("section", terminated(preceded(multispace0, tuple((
        opt(parse_label),
        opt(parse_reps),
        alt((parse_distance, parse_subset)),
        opt(parse_attributes),
    ))), multispace0),
    )(i)?;

    let data = SetData {
        label: match e.0 {
            None => None,
            Some(SetComponent::Label(l)) => Some(l),
            _ => unreachable!()
        },
        reps: match e.1 {
            None => 1,
            Some(SetComponent::Reps(r)) => r,
            _ => unreachable!()
        },
        attributes: match e.3 {
            None => vec![],
            Some(SetComponent::Attributes(a)) => a.clone(),
            _ => unreachable!()
        },
    };


    match e.2 {
        SetComponent::Subset(subsets) => {
            Ok((
                i,
                Set::Subset { subsets, data }
            ))
        }

        SetComponent::Distance(distance) => {
            Ok((
                i,
                Set::Single { distance, data }
            ))
        }
        _ => unreachable!()
    }
}

fn parse_text(i: &str) -> IResult<&str, Set, VerboseError<&str>> {
    map(
        context("text", preceded(multispace0, terminated(
            preceded(
                space0,
                preceded(
                    char('-'),
                    preceded(space0, is_not("\n\r"),
                    ))), multispace0))),
        |s: &str| Set::Text(s.to_string()))(i)
}

fn root_parser(i: &str) -> IResult<&str, Vec<Set>, VerboseError<&str>> {
    many1(alt((parse_text, parse_section)))(i)
}

pub fn parse<'a>(mut i: String) -> IResult<&'a str, Vec<Set>, VerboseError<&'a str>> {
    i.push('\n');
    let result = root_parser(i.as_str());
    i.pop();
    result

}

