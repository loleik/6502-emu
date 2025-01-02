use crate::trie::Trie;

// Main disassembler function. Takes the binary vector as input.
// Jumps and such won't work as I'm just plainly going through the binary
// instruction by instruction, only ensuring we jump past any addresses or data.
pub fn disassembler(data: &Vec<u8>, prefix_trie: &Trie) {
    let mut i: usize = 0;

    // I feel like this is messy and cumbersome.
    // Loop through all the provided data.
    while i < data.len() {
        if let Some(current) = prefix_trie.get_instruction(data[i]) {
            let arr: Vec<&str> = current.split(",").collect();

            // Memory addresses are stored as little endian values.
            // I am honestly not sure if I'm handling addresses in the right endian or not..
            match arr[1] {
                "ABS" => { // Absolute
                    println!(
                        "{} ${:04X}",
                        arr[0],
                        ((data[i + 1] as u16) << 8) | (data[i + 2] as u16)
                    )
                }
                "ABSX" | "ABSY" => { // Absolute X and Y
                    println!(
                        "{} ${:04X},{}",
                        arr[0],
                        ((data[i + 1] as u16) << 8) | (data[i + 2] as u16),
                        arr[1].chars().last().unwrap()
                    )
                }
                "IND" => { // Indirect
                    println!(
                        "{} (${:04X})",
                        arr[0],
                        ((data[i + 1] as u16) << 8) | (data[i + 2] as u16),
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
                        data[i + 1],
                    )
                }
                "ZP" => { // Zero Page
                    println!(
                        "{} ${:02X}",
                        arr[0],
                        data[i + 1],
                    )
                }
                "ZPX" | "ZPY" => { // Zero Page X and Y
                    println!(
                        "{} ${:02X},{}",
                        arr[0],
                        data[i + 1],
                        arr[1].chars().last().unwrap(),
                    )
                }
                "INDX" => { // Indexed Indirect
                    println!(
                        "{} (${:02X},X)",
                        arr[0],
                        data[i + 1],
                    )
                }
                "INDY" => { // Indirect Indexed
                    println!(
                        "{} (${:02X}),Y",
                        arr[0],
                        data[i + 1],
                    )
                }
                "REL" => { // Relative
                    println!(
                        "{} ${:02X}",
                        arr[0],
                        data[i + 1] as i8,
                    )
                }
                _ => println!(
                        "{:?} : {:?} : {:02X}",
                        arr[0], arr[1],
                        data[i]
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