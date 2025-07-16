use codegen::to_asm;
use std::{env, fs, process::Command};
use token::tokenize;

mod codegen;
mod token;

fn main() {
    let file_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No file path argument provided.\nUsage: letterscript <file.lts>");
        std::process::exit(1);
    });

    let contents = fs::read_to_string(&file_path).unwrap_or_else(|err| {
        eprintln!("Could not read file at '{file_path}': {err}");
        std::process::exit(1);
    });

    let asm_code = to_asm(tokenize(&contents));

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
