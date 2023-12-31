use std::collections::HashMap;
use std::env;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::DATA_FILE_NAME;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn make_hashmap(path: &str) -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    if let Ok(lines) = read_lines(&path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(project) = line {
                // Split
                let line_content: Vec<&str> = project.split_whitespace().collect();

                if line_content.len() == 2 {
                    aliases.insert(line_content[0].to_owned(), line_content[1].to_owned());
                } else {
                    // Handle the case when there are not enough elements in line_content
                    // You may want to log an error, skip the line, or handle it differently based on your needs.
                    panic!("Wring format.")
                }
            }
        }
    }
    aliases
}

fn rewrite_file(data: &mut HashMap<String, String>) -> Result<(), io::Error> {
    let data_file_path = format!("{}/{}", env::var("PROJUMP").unwrap(), DATA_FILE_NAME);
    let mut file = File::create(&data_file_path)?;
    for (a, p) in data.iter() {
        writeln!(file, "{} {}", a, p)?;
    }

    Ok(())
}

pub fn list() {
    let data_file_path = format!("{}/{}", env::var("PROJUMP").unwrap(), DATA_FILE_NAME);
    let contents = read_to_string(&data_file_path).expect("Should have been able to read the file");
    println!("{}", contents)
}

pub fn delete(data: &mut HashMap<String, String>, key: &str) {
    data.remove(key);
    let _ = rewrite_file(data);
}

pub fn update(data: &mut HashMap<String, String>, old_key: &str, new_key: &str) {
    match data.get(old_key) {
        None => println!("No alias by the name {}", old_key),
        Some(v) => {
            data.insert(new_key.to_owned(), v.to_owned());
            data.remove(old_key);
        }
    }
    let _ = rewrite_file(data);
}

pub fn set(data: &mut HashMap<String, String>, key: &str, path: String, force: bool) {
    match data.get(key) {
        None => {
            data.insert(key.to_owned(), path);
        }
        Some(_v) => {
            if force == true {
                data.insert(key.to_owned(), path);
            } else {
                eprintln!("Alias \'{}\' already exists. Use \'--force\' to override.", key);
            }
        }
    }
    let _ = rewrite_file(data);
}
