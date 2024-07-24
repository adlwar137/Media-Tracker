use std::env;
use std::fs;
use std::collections::HashMap;

pub mod parser;

#[derive(Debug)]
enum MediaType {
    Movie,
    Book,
    Show,
}

#[derive(Debug)]
struct MediaEntry {
   mediaType: MediaType,
   name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    dbg!(load_file(file_path.to_string()));

}

fn load_file(file_path: String) -> MediaEntry {
    let raw_file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = HashMap::new();

    for line in parser::parse_text(raw_file) {
        lines.insert(line[0].clone(), line[1].clone());
    }

    file_to_entry(lines)
}

fn file_to_entry(file: HashMap<String, String>) -> MediaEntry {
    MediaEntry {
        mediaType: match file.get("mediaType") {
            None => panic!("asdfasdfasdf"),
            Some(thing) => match thing.as_str() {
                "Movie" => MediaType::Movie,
                "Book" => MediaType::Book,
                "Show" => MediaType::Show,
                _ => panic!("asdfasdfasdf"),
            },
        },
        name: match file.get("name") {
            None => panic!("asdfasdfasdf"),
            Some(thing) => thing.to_string(),
        },
    }
}
