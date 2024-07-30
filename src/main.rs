use std::fs;
use std::collections::HashMap;
use std::io;
use crate::media_list::media_entry::*;
use crate::media_list::*;
use crate::file_handling::*;
use std::str::FromStr;
use std::env;

pub mod parser;
pub mod file_handling;
pub mod media_list;



fn main() {
    //TODO check arguments to load a file
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(x) => x,
        None => panic!("asdfasdf"),
    };


    let mut stuff = MediaList::new();

    loop {
        let reply = ask("L) To list entries\nA) To add an entry\nR) To remove an entry\nM) To modify an entry\nS) To save entries\nQ) To quit");

        match reply.to_lowercase().as_str() {
            "l" => list_entries(&mut stuff),
            "a" => add_entry(&mut stuff),
            //"r" => remove_entry(&mut entries),
            //"m" => modify_entry(&mut entries),
            "s" => save_file(stuff.clone(), &file_path),
            "lo" => file_load(&file_path),
            "q" => break,
            _ => println!("whar??? in: ({reply})"),
        };
    }
}

fn ask(query: &str) -> String {
    println!("{}", query);

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer.trim().to_string()
}

fn add_entry(list: &mut MediaList) {
    list.add_entry(MediaEntry {
        mediaType: MediaType::from_str(&ask("What is the type of media?")).expect("Whar?"),
        name: ask("What is the title?"),
    });
}

fn list_entries(list: &mut MediaList) {
    list.list_entries();
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
