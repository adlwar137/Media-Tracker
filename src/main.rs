use std::env;
use std::fs;
use std::collections::HashMap;
use std::io;
use crate::media_entry::*;
use crate::file_handling::*;
use std::str::FromStr;

pub mod parser;
pub mod file_handling;
pub mod media_entry;



fn main() {
    //TODO check arguments to load a file




    // let mut entries: Vec<MediaEntry> = Vec::new();
    
    // loop {
    //     let reply = ask("A) To add an entry\nL) To list entries\nM) To modify an entry\nS) To save entries\nQ) To quit");

    //     match reply.to_lowercase().as_str() {
    //         "a" => add_entry(&mut entries),
    //         "l" => list_entries(&entries),
    //         "m" => modify_entry(&mut entries),
    //         "s" => println!("saving..."),
    //         "q" => break,
    //         _ => println!("whar??? in: ({reply})"),
    //     };
    // }

    file_load();
}

fn ask(query: &str) -> String {
    println!("{}", query);

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer.trim().to_string()
}

fn add_entry(entryList: &mut Vec<MediaEntry>) {
    let name = ask("What is the title?");

    let media = ask("What is the type of media?");

    let entry = MediaEntry {
        mediaType: MediaType::from_str(&media).expect("dass da wrong numbaaa"),
        name,
    };

    entryList.push(entry);
}

fn list_entries(entry_list: &Vec<MediaEntry>) {
    for entry in entry_list {
        println!("Entry:");
        println!("\tName: {}", entry.name);
        println!("\tType: {}", entry.mediaType);
    }
}

fn modify_entry(entry_list: &mut Vec<MediaEntry>) {
    //decide whether or not to construct a new one
}

fn remove_entry(entry_list: &mut Vec<MediaEntry>) {
    //search for name and then delete
    let query = ask("what is the title of the entry you want to delete?");
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
