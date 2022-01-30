use std::fs;
mod parser;

fn get_data(file: &str) -> String{
    let mut result = fs::read_to_string(file).expect("Could not read file.");
    result.push('\n');
    result
}

fn main() {
    let data = get_data("test.swim");
    let result = parser::parser(&data).unwrap().1;

    let json = serde_json::to_string(&result);
    fs::write("test.json", json.unwrap());
    println!("{:#?}", result);
}
