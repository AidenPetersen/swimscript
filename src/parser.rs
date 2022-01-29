use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric0, char, line_ending, multispace0, newline, not_line_ending, space0, u32};
use nom::combinator::{map, cut, eof};
use nom::error::VerboseError;
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub enum SetComponent {
    Label(String),
    Reps(u32),
    Distance(u32),
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

fn components_to_set(components: Vec<SetComponent>) -> Set {
    let mut data = SetData {
        label: None,
        reps: 1,
        attributes: vec![],
    };
    let mut dist = 0;
    components.iter().for_each(|c| {
        match c {
            SetComponent::Label(l) => { data.label = Some(l.to_string()) }
            SetComponent::Reps(r) => { data.reps = *r }
            SetComponent::Distance(d) => { dist = *d }
        }
    });
    Set::Single { distance: dist, data }
}

pub fn parse_label(i: &str) -> IResult<&str, SetComponent> {
    map(terminated(terminated(is_not(":\n"), char(':')), space0),
        |s: &str| SetComponent::Label(s.to_string()),
    )(i)
}

pub fn parse_reps(i: &str) -> IResult<&str, SetComponent> {
    map(terminated(
        preceded(
            space0,
            terminated(u32, preceded(space0, char('*')),
            )), space0),
        |n: u32| SetComponent::Reps(n))(i)
}

pub fn parse_distance(i: &str) -> IResult<&str, SetComponent> {
    map(preceded(space0, terminated(u32, space0)),
        |n: u32| SetComponent::Distance(n))(i)
}


pub fn parse_section(i: &str) -> IResult<&str, Set> {
    let (i, e) = preceded(
         multispace0,
            terminated(many0(alt((parse_label, parse_reps, parse_distance))), newline))(i)?;
    let set = components_to_set(e);
    Ok((
        i,
        set
    ))
}

pub fn parse_text(i: &str) -> IResult<&str, Set> {
    map(terminated(
        preceded(
            space0,
            preceded(
                char('-'),
                preceded(space0, is_not("\n\r"),
                ))), multispace0),
        |s: &str| Set::Text(s.to_string()))(i)
}

pub fn parsertest(i: &str) -> IResult<&str, Vec<SetComponent>> {
    many0(alt((parse_label, parse_reps, parse_distance)))(i)
}

pub fn parser(i: &str) -> IResult<&str, Vec<Set>> {
    many0(alt((parse_section, parse_text)))(i)
}
