use std::env;
use std::fs::File;
use std::io::ErrorKind;
use projump::actions;
use projump::{ROOT, DATA_FILE_NAME};
use std::process;
use std::path::Path;

fn main() {
    // Check data file
    let data_file_path: String = ROOT.to_owned() + "/" + DATA_FILE_NAME;
    match File::open(&data_file_path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("File not found. Creating a new file.");

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
                if command.contains("--force") {
                    force = true;
                }
                actions::set(&mut aliases, &args[1], cwd, force)
            },
            "-d" => actions::delete(&mut aliases, &args[1]),
            "-m" => actions::update(&mut aliases, &args[1], &args[2]),
            v => {
                // check valid path
                let path_input: &Path = Path::new(v);
                let is_valid: bool = path_input.is_dir();
                if is_valid == true && &args[1] == "-a" {
                    let mut force: bool = false;
                    if command.contains("--force") {
                        force = true;
                    }
                    // 절대 경로인 경우
                    actions::set(&mut aliases, &args[2], args[0].to_owned(), force)

                    // TODO: 상대경로인 경우
                    
                } else {
                    eprintln!("No such directory: {}", v); 
                    process::exit(1);
                }

               
            },
        }
    }
}