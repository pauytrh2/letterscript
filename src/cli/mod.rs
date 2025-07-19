use std::env;

pub struct Args {
    pub file_path: String,
    pub is_dev: bool,
}

pub fn parse_cli_args() -> Args {
    let file_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No file path argument provided.\nUsage: letterscript <file.lts>");
        std::process::exit(1);
    });

    let is_dev = env::args().nth(2).unwrap_or_default() == "--dev";

    Args { file_path, is_dev }
}
