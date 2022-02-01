use crate::to_json;

const BASIC_JSON: &str = "[{\"Subset\":{\"subsets\":[{\"Single\":{\"distance\":125,\"data\":{\"label\":null,\"reps\":5,\"attributes\":[\"75 swim\",\"@1:45\",\"25 drill\",\"25 kick\"]}}},{\"Single\":{\"distance\":200,\"data\":{\"label\":null,\"reps\":1,\"attributes\":[\"IM\",\"Drill / Swim\"]}}},{\"Single\":{\"distance\":50,\"data\":{\"label\":null,\"reps\":4,\"attributes\":[\"Choice\",\"@:55\",\"Variable\"]}}}],\"data\":{\"label\":\"Warm Up\",\"reps\":1,\"attributes\":[]}}}]";
#[test]
fn it_works() {
    assert_eq!(1, 1)
}

#[test]
fn basic() {
    assert_eq!(to_json(r#"
Warm Up: {
    5 * 125 75 swim, @1:45 , 25 drill, 25 kick
    200 IM, Drill / Swim
    4 * 50 Choice, @:55, Variable
}
    "#).unwrap(), BASIC_JSON)
}

#[test]
fn no_newline() {
    assert_eq!(to_json(r#"
Warm Up: {
    5 * 125 75 swim, @1:45 , 25 drill, 25 kick
    200 IM, Drill / Swim
    4 * 50 Choice, @:55, Variable
}"#).unwrap(), BASIC_JSON)
}

#[test]
fn no_newline_after_bracket() {
    assert_eq!(to_json(r#"
Warm Up: {
    5 * 125 75 swim, @1:45 , 25 drill, 25 kick
    200 IM, Drill / Swim
    4 * 50 Choice, @:55, Variable}"#).unwrap(), BASIC_JSON)
}

#[test]
fn no_newline_before_bracket() {
    assert_eq!(to_json(r#"
Warm Up: { 5 * 125 75 swim, @1:45 , 25 drill, 25 kick
    200 IM, Drill / Swim
    4 * 50 Choice, @:55, Variable
}
"#).unwrap(), BASIC_JSON)
}

#[test]
fn newline_after_label() {
    assert_eq!(to_json(r#"
Warm Up:
{
    5 * 125 75 swim, @1:45 , 25 drill, 25 kick
    200 IM, Drill / Swim
    4 * 50 Choice, @:55, Variable
}
"#).unwrap(), BASIC_JSON)
}


#[test]
fn minimum_newlines() {
    assert_eq!(to_json(r#"
Warm Up: {5 * 125 75 swim, @1:45 , 25 drill, 25 kick
          200 IM, Drill / Swim
          4 * 50 Choice, @:55, Variable}"#).unwrap(), BASIC_JSON)
}

#[test]
fn basic_no_whitespace() {
    assert_eq!(to_json(r#"
Warm Up:{
    5 * 125 75 swim,@1:45,25 drill,25 kick
    200 IM,Drill / Swim
    4 * 50 Choice,@:55,Variable
}
"#).unwrap(), BASIC_JSON)
}

#[test]
fn basic_no_whitespace_or_newline() {
    assert_eq!(to_json(r#"
Warm Up:{5 * 125 75 swim,@1:45,25 drill,25 kick
    200 IM,Drill / Swim
    4 * 50 Choice,@:55,Variable}"#).unwrap(), BASIC_JSON)
}