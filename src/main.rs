use lolei_6502::{disassembler::disassembler, trie::{gen_trie, Trie}};

use std::fs;
use clap::{arg, Command};

// Basically the git example from https://github.com/clap-rs/clap/tree/master/examples.
// Decided to get this implemented earlier than with chip-8.
fn cli() -> Command {
    Command::new("lolei_6502")
        .about("Emulator and disassembler for the MOS Technology 6502 microprocessor")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        // Subcommand for the disassembler.
        .subcommand( // Expects a path to a binary file.
            Command::new("disassemble")
                .about("Disassemble binaries")
                .arg(arg!(<PATH> "The binary to disassemble"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let prefix_trie: Trie = gen_trie();

    let matches = cli().get_matches();

    match matches.subcommand() {
        // Executing the disassembler subcommand.
        Some(("disassemble", sub_matches)) => {
            println!(
                "Disassembling {}:",
                sub_matches.get_one::<String>("PATH").expect("required")
            );

            let data: Vec<u8> = match fs::read("6502_decimal_test.bin") {
                Ok(data) => data,
                Err(error) => panic!("Problem opening file: {error:?}")
            };
        
            disassembler(&data, &prefix_trie);
        }
        _ => {unreachable!()}
    }
}