use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric0, char, line_ending, multispace0, newline, not_line_ending, space0, u32};
use nom::combinator::{map, cut, eof, opt};
use nom::error::{context, VerboseError};
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::sequence::terminated;
use nom::{IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub enum SetComponent {
    Label(String),
    Reps(u32),
    Distance(u32),
    Attributes(Vec<String>),
    Subset(Vec<Set>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetData {
    label: Option<String>,
    reps: u32,
    attributes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Set {
    Multiline { subsets: Vec<Set>, data: SetData },
    Single { distance: u32, data: SetData },
    Text(String),
}

fn parse_label(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context(
            "label", terminated(terminated(is_not(":\n"), char(':')), space0),
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
                preceded(multispace0, preceded(char('{'), terminated(parser, char('}'))))),
        |x| SetComponent::Subset(x))(i)
}

fn parse_attributes(i: &str) -> IResult<&str, SetComponent, VerboseError<&str>> {
    map(
        context("attributes",
                preceded(
                    space0, separated_list1(
                        char(','), preceded(space0, is_not(",\n\r")))),
        ),
        |x: Vec<&str>| SetComponent::Attributes(x.iter().map(|y| y.to_string()).collect()),
    )(i)
}


// fn parse_section(i: &str) -> IResult<&str, Set> {
//     let (i, e) = preceded(
//         multispace0,
//         terminated(many0(alt((parse_label, parse_reps, parse_distance, parse_attributes))), newline))(i)?;
//     let set = components_to_set(e);
//     Ok((
//         i,
//         set
//     ))
// }
fn parse_section(i: &str) -> IResult<&str, Set, VerboseError<&str>> {
    let (i, e) = context("section", preceded(multispace0, terminated(tuple((
        opt(parse_label),
        opt(parse_reps),
        alt((parse_distance, parse_subset)),
        opt(parse_attributes),
    )), newline),
    ))(i)?;

    let mut data = SetData {
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
            return Ok((
                i,
                Set::Multiline { subsets, data }
            ));
        }
        SetComponent::Distance(distance) => {
            return Ok((
                i,
                Set::Single { distance, data }
            ));
        }
        _ => unreachable!()
    }
}

fn parse_text(i: &str) -> IResult<&str, Set, VerboseError<&str>> {
    map(
        context("text", terminated(
            preceded(
                space0,
                preceded(
                    char('-'),
                    preceded(space0, is_not("\n\r"),
                    ))), multispace0)),
        |s: &str| Set::Text(s.to_string()))(i)
}

pub fn parser(i: &str) -> IResult<&str, Vec<Set>, VerboseError<&str>> {
    many0(alt((parse_text, parse_section)))(i)
}
