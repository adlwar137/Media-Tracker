use std::fs::File;
use std::fs;
use std::io::Write;
use crate::MediaList;

pub fn file_load(file_path: &str, list: &mut MediaList) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let parsed = json::parse(&contents).expect("failed to parse");

    *list = MediaList::from(parsed);
}

pub fn save_file(contents: MediaList, file_path: &str) {
    let serialized = json::stringify(contents);
     
    let mut f = File::create(file_path).expect("file could not be created");
    f.write_all(serialized.as_bytes()).expect("file could not be written");
}
