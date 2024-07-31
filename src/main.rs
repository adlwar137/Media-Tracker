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

    file_load(file_path, &mut stuff);


    loop {
        let reply = ask("L) To list entries\nA) To add an entry\nR) To remove an entry\nM) To modify an entry\nS) To save entries\nQ) To quit");

        match reply.to_lowercase().as_str() {
            "l" => list_entries(&mut stuff),
            "a" => add_entry(&mut stuff),
            "r" => remove_entry(&mut stuff),
            //"m" => modify_entry(&mut entries),
            "s" => save_file(stuff.clone(), file_path),
            "lo" => file_load(file_path, &mut stuff),
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

fn remove_entry(list: &mut MediaList) {
    let name = ask("What is the title?");

    list.remove_entry(&name);
}
