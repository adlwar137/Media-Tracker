use std::fs::File;
use std::io::Read;
use std::env;

//ipv4 185.107.80.231
//ipv6 2001:0000:130F:0000:0000:09C0:876A:130B

pub fn file_load() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {file_path}");

    let mut data_file = File::open(file_path).unwrap();

    let mut file_content = String::new();

    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);
}

