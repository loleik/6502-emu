use crate::trie::Trie;

// Very basic system struct for the purpose of disassembly.
// The branch and jump instructions need a basic implementation to access all code of a program.
struct BasicSystem {
    memory: [u8; 16384], // 16kb of memory
    pc: u16, // 16-bit program counter
}

// Function for generating the basic struct.
impl BasicSystem {
    fn new(start: &u16) -> Self {
        Self {
            memory: [0; 16384],
            pc: *start,
        }
    }
}

// Main disassembler function. Takes the binary vector as input.
// Jumps and such won't work as I'm just plainly going through the binary
// instruction by instruction, only ensuring we jump past any addresses or data.
pub fn disassembler(data: &Vec<u8>, start: &u16, prefix_trie: &Trie) {
    let mut i: usize = *start as usize;

    // Initialize the basic system.
    let mut basic_system = BasicSystem::new(start);

    // Load the data into memory.
    basic_system.memory[
        (basic_system.pc as usize)..(basic_system.pc as usize + data.len())
    ].copy_from_slice(data);

    // I feel like this is messy and cumbersome.
    // Loop through all the provided data.
    while i < basic_system.pc as usize + data.len() {
        if let Some(current) = prefix_trie.get_instruction(basic_system.memory[i]) {
            let arr: Vec<&str> = current.split(",").collect();

            // Memory addresses are stored as little endian values.
            // I am honestly not sure if I'm handling addresses in the right endian or not..
            match arr[1] {
                "ABS" => { // Absolute
                    println!(
                        "{} ${:04X}",
                        arr[0],
                        ((basic_system.memory[i + 1] as u16) << 8) 
                        | (basic_system.memory[i + 2] as u16)
                    )
                }
                "ABSX" | "ABSY" => { // Absolute X and Y
                    println!(
                        "{} ${:04X},{}",
                        arr[0],
                        ((basic_system.memory[i + 1] as u16) << 8) 
                        | (basic_system.memory[i + 2] as u16),
                        arr[1].chars().last().unwrap()
                    )
                }
                "IND" => { // Indirect
                    println!(
                        "{} (${:04X})",
                        arr[0],
                        ((basic_system.memory[i + 1] as u16) << 8) 
                        | (basic_system.memory[i + 2] as u16),
                    )
                }
                "IMP" => { // Implicit
                    println!(
                        "{}",
                        arr[0],
                    )
                }
                "ACC" => { // Accumulator
                    println!(
                        "{} A",
                        arr[0],
                    )
                }
                "IMM" => { // Immediate
                    println!(
                        "{} #{:02X}",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                "ZP" => { // Zero Page
                    println!(
                        "{} ${:02X}",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                "ZPX" | "ZPY" => { // Zero Page X and Y
                    println!(
                        "{} ${:02X},{}",
                        arr[0],
                        basic_system.memory[i + 1],
                        arr[1].chars().last().unwrap(),
                    )
                }
                "INDX" => { // Indexed Indirect
                    println!(
                        "{} (${:02X},X)",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                "INDY" => { // Indirect Indexed
                    println!(
                        "{} (${:02X}),Y",
                        arr[0],
                        basic_system.memory[i + 1],
                    )
                }
                // Only the branch functions use relative addressing.
                "REL" => { // Relative
                    println!(
                        "{} ${:02X}",
                        arr[0],
                        basic_system.memory[i + 1] as i8,
                    )
                }
                _ => println!(
                        "{:?} : {:?} : {:02X}",
                        arr[0], arr[1],
                        basic_system.memory[i]
                )
            }

            if arr[2].parse::<usize>().unwrap() > 1 {
                i += arr[2].parse::<usize>().unwrap();
            } else {
                i += 1
            }
        } else {
            i += 1
        }
    }
}