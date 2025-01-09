use crate::trie::Trie;

use std::fs::File;
use std::io::prelude::*;

// Very basic system struct for the purpose of disassembly.
// Thought I needed this but it really seems like I didn't at all.
// I'll leave it for now and remove it later.
struct BasicSystem {
    memory: [u8; 65536], // 64kb of memory
    pc: u16, // 16-bit program counter
}

// Function for generating the basic struct.
impl BasicSystem {
    fn new(start: &u16) -> Self {
        Self {
            memory: [0; 65536],
            pc: *start,
        }
    }
}

// Main disassembler function. Takes the binary vector as input.
// Jumps and such won't work as I'm just plainly going through the binary
// instruction by instruction, only ensuring we jump past any addresses or data.
pub fn disassembler(
    data: &Vec<u8>, 
    start: &u16, 
    prefix_trie: &Trie
) -> std::io::Result<()> {
    let mut i: usize = *start as usize;
    let mut to_visit = Vec::new();

    let mut file = File::create("out.txt")?;

    // Initialize the basic system.
    let mut basic_system = BasicSystem::new(start);

    // Load the data into memory.
    basic_system.memory[
        (basic_system.pc as usize)..(basic_system.pc as usize + data.len())
    ].copy_from_slice(data);

    basic_system.pc = 0x0200;

    // I feel like this is messy and cumbersome.
    // Loop through all the provided data.
    while i < basic_system.pc as usize + data.len() {
        if let Some(current) = prefix_trie.get_instruction(basic_system.memory[i]) {
            let arr: Vec<&str> = current.split(",").collect();

            if arr[0] == "JMP" || arr[0] == "JSR" {
                to_visit.push(
                    ((basic_system.memory[i + 1] as u16) << 8) 
                    | (basic_system.memory[i + 2] as u16)
                );
            }

            // Memory addresses are stored as little endian values.
            // I am honestly not sure if I'm handling addresses in the right endian or not..
            let line: String = match arr[1] {
                "ABS" => { // Absolute
                    format!(
                        "{} ${:04X} \n",
                        arr[0],
                        ((basic_system.memory[i + 1] as u16) << 8) 
                        | (basic_system.memory[i + 2] as u16)
                    )
                }
                "ABSX" | "ABSY" => { // Absolute X and Y
                    format!(
                        "{} ${:04X},{} \n",
                        arr[0],
                        ((basic_system.memory[i + 1] as u16) << 8) 
                        | (basic_system.memory[i + 2] as u16),
                        arr[1].chars().last().unwrap()
                    )
                }
                "IND" => { // Indirect
                    format!(
                        "{} (${:04X}) \n",
                        arr[0],
                        ((basic_system.memory[i + 1] as u16) << 8) 
                        | (basic_system.memory[i + 2] as u16),
                    )
                }
                "IMP" => { // Implicit
                    format!(
                        "{} \n",
                        arr[0],
                    )
                }
                "ACC" => { // Accumulator
                    format!(
                        "{} A \n",
                        arr[0],
                    )
                }
                "IMM" => { // Immediate
                    format!(
                        "{} #{:02X} \n",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                "ZP" => { // Zero Page
                    format!(
                        "{} ${:02X} \n",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                "ZPX" | "ZPY" => { // Zero Page X and Y
                    format!(
                        "{} ${:02X},{} \n",
                        arr[0],
                        basic_system.memory[i + 1],
                        arr[1].chars().last().unwrap(),
                    )
                }
                "INDX" => { // Indexed Indirect
                    format!(
                        "{} (${:02X},X) \n",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                "INDY" => { // Indirect Indexed
                    format!(
                        "{} (${:02X}),Y \n",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                // Only the branch functions use relative addressing.
                "REL" => { // Relative
                    format!(
                        "{} ${:02X} \n",
                        arr[0],
                        basic_system.memory[i + 1] as i8,
                    )
                }
                _ => format!(
                        "{:?} : {:?} : {:02X} \n",
                        arr[0], arr[1],
                        basic_system.memory[i]
                )
            };

            print!("{line}");
            file.write_all(line.as_bytes())?;

            if arr[2].parse::<usize>().unwrap() > 1 {
                i += arr[2].parse::<usize>().unwrap();
            } else {
                i += 1
            }
        } else {
            i += 1
        }
    }

    Ok(())
}