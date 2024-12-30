use lolei_6502::{disassembler::disassembler, trie::DisTrie};

use std::fs;

// Messy "disassembler" to start out. It gives expected outputs so far.
fn main() {
    let data: Vec<u8> = match fs::read("6502_decimal_test.bin") {
        Ok(data) => data,
        Err(error) => panic!("Problem opening file: {error:?}")
    };

    let dis_trie: DisTrie = disassembler();

    let mut i: usize = 0;

    while i < 20 {
        println!("{}", data[i]);
        println!("{:?} : {:02X}",
            dis_trie.get_instruction(data[i]),
            data[i]);
        if let Some(current) = dis_trie.get_instruction(data[i]) {
            let arr: Vec<&str> = current.split(",").collect();
            println!("{} {} {} \n", arr[2], arr[3], arr[4]);
            if arr[2].parse::<usize>().unwrap() > 1 {
                i += arr[2].parse::<usize>().unwrap();
            } else {
                i += 1
            }
        };
    }
}