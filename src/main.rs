use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!(
                "Error: No file path argument provided.\nPlease provide a file path.\nletterscript <file.lts>"
            );
            std::process::exit(1);
        }
    };

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    dbg!(contents);
}
