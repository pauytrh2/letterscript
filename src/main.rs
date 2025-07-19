use cli::parse_cli_args;
use codegen::to_asm;
use parse::Parser;
use std::{fs, process::Command};
use token::tokenize;

mod cli;
mod codegen;
mod parse;
mod token;

fn main() {
    let args = parse_cli_args();

    let contents = fs::read_to_string(&args.file_path).unwrap_or_else(|err| {
        eprintln!("Could not read file at '{}': {err}", args.file_path);
        std::process::exit(1);
    });

    let tokens = tokenize(&contents);
    if args.is_dev {
        dbg!(&tokens);
    }

    let mut parser = Parser::new(tokens);
    let parse_tree = parser.parse_program().expect("Error parsing");
    if args.is_dev {
        dbg!(&parse_tree);
    }

    let asm_code = to_asm(parse_tree);

    fs::write("output.asm", asm_code).expect("Unable to write to file");

    Command::new("nasm")
        .args(["-f", "elf64"])
        .args(["-o", "output.o"])
        .arg("output.asm")
        .spawn()
        .expect("nasm failed")
        .wait()
        .expect("nasm wait failed");

    Command::new("ld")
        .args(["-o", "output"])
        .arg("output.o")
        .spawn()
        .expect("ld failed")
        .wait()
        .expect("ld wait failed");
}
