use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::process;

use projump::actions;
use projump::DATA_FILE_NAME;

fn main() {
    // Check data file
    let data_file_path = format!("{}/{}", env::var("PROJUMP").unwrap(), DATA_FILE_NAME);
    match File::open(&data_file_path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                // Create a new file if not found
                match File::create(&data_file_path) {
                    Ok(new_file) => new_file,
                    Err(e) => panic!("{}", e),
                }
            }
            _ => panic!("{}", e),
        },
    };

    let mut aliases = actions::make_hashmap(&data_file_path);

    // Command line arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // First argument
    if let Some(command) = args.first() {
        match command.as_str() {
            "ls" => actions::list(),
            "-a" => {
                let cwd: String = env::current_dir().unwrap().to_string_lossy().into_owned();
                let mut force: bool = false;
                if args[args.len() - 1] == "--force" {
                    force = true;
                }
                actions::set(&mut aliases, &args[1], cwd, force)
            }
            "-d" => actions::delete(&mut aliases, &args[1]),
            "-m" => actions::update(&mut aliases, &args[1], &args[2]),
            v => {
                // check valid path
                let path: &Path = Path::new(v);
                let is_valid: bool = path.is_dir();

                if is_valid == true {
                    if &args[1] == "-a" {
                        let mut force: bool = false;
                        if args[args.len() - 1] == "--force" {
                            force = true;
                        }

                        if path.is_absolute() {
                            actions::set(&mut aliases, &args[2], args[0].to_owned(), force)
                        } else {
                            actions::set(&mut aliases, &args[2], path.canonicalize().unwrap().to_str().unwrap().to_string(), force)
                        }
                    } else {
                        eprintln!("Unknown command: {}", command);
                        process::exit(1);
                    }
                } else {
                    // Path not valid
                    eprintln!("No such directory: {}", v);
                    process::exit(1);
                }
            }
        }
    }
}