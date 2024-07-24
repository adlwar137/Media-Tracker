use std::env;
use std::fs;
use std::collections::HashMap;
use std::io;

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
    //TODO check arguments to load a file


    let mut entries: Vec<MediaEntry> = Vec::new();
    
    loop {
        let reply = ask("A) To add an entry");

        match reply.to_lowercase().as_str() {
            "a" => add_entry(&mut entries),
            "m" => println!("modify..."),
            "s" => println!("saving..."),
            "l" => println!("loading..."),
            _ => println!("whar??? in: ({reply})"),
        };

        dbg!(&entries);
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

fn add_entry(entryList: &mut Vec<MediaEntry>) {
    let name = ask("What is the title?");

    let media = ask("What is the type of media?").to_lowercase();

    let entry = MediaEntry {
        mediaType: match media.as_str() {
            "book" => MediaType::Book,
            "movie" => MediaType::Movie,
            "show" => MediaType::Show,
            _ => {
                println!("invalid media type");
                return;
            },
        },
        name
    };

    entryList.push(entry);
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
