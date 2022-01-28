use std::fs;
use nom::Finish;

mod parser;

fn get_data(file: &str) -> String{
    fs::read_to_string(file).expect("Could not read file.")
}

fn main() {
    let data = get_data("test.swim");
    let result = parser::parser(&data).finish();

    println!("{:#?}", result);
}
