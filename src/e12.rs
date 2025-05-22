use clap::{Arg, Command};
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    filename: String,
    content: String,
}

pub fn mainfn() {
    let matches = Command::new("fm")
        .version("1.0")
        .about("File manager")
        .arg(
            Arg::new("file_name")
                .help("Name of the file to lookup")
                .required(true)
                .index(1)
        )
        .get_matches();


    let data = fs::read_to_string("jsontemp.txt")
        .expect("Failed to read jsontemp.txt");
    let parsed: HashMap<String, Data> = serde_json::from_str(&data)
        .expect("Failed to parse JSON");
    let file_name = matches.get_one::<String>("file_name").unwrap();
    match parsed.get(file_name) {
        Some(data) => {
            println!("Filename: {}", data.filename);
            println!("Content: {}", data.content);
        },
        None => eprintln!("No entry found for '{}'", file_name),
    }
}