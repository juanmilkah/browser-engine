use std::fs;
use std::io::Read;

use browser_engine::parse;

fn main() {
    println!("Hello, world!");

    let filepath = "./index.html";
    let content = read_file(filepath);
    let root_element = parse(content);
    println!("{:?}", root_element);
}

fn read_file(filepath: &str) -> String {
    let mut file = fs::OpenOptions::new().read(true).open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
