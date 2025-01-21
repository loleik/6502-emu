use crate::trie::Trie;

use std::io::{self, Write};

pub struct Core {
    pub acc: u8, // 8-bit accumulator register
    pub stat: u8, // 7-bit status register, stored as u8
    // Negative, Overflow, Ignored, Break, Decimal, Interrupt, Zero, Carry.
    pub pc: u16, // 16-bit program counter
    pub sp: u8, // 8-bit stack pointer
    // The stack is conventionally at 0x0100-0x01FF in memory
    pub ix: u8, // 8-bit index register
    pub iy: u8, // 8-bit index register
    pub ir: u8, // 8-bit instruction register
    pub decoded: Option<fn(&mut Core) -> &mut Core>, // Stores opcode funciton pointer
    pub info: Option<String>, // Opcode infor string, mainly for core dump function.
    // Note: This doesn't align with any particular systems, it is just enough to 
    // load specific 6502 test binaries.
    pub memory: [u8; 65536], // 64kb of memory
}

impl Core {
    pub fn new() -> Self {
        Self {
            acc: 0,
            stat: 0,
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
            ir: 0,
            decoded: None,
            info: None,
            memory: [0; 65536],
        }
    }

    // Function for dumping the state of the core.
    // No display built in to the 6502, therefore this will be useful
    // for debugging and testing later.
    pub fn core_dump(core: &Self) {
        //print!("\x1B[2J\x1B[1;1H");
        //io::stdout().flush().unwrap();

        println!("-->core dump<--");
        println!("acc:     0x{:02X}", core.acc);
        println!("stat:    0b{:08b}", core.stat);
        println!("pc:      0x{:04X}", core.pc);
        println!("sp:      0x{:02X}", core.sp);
        println!("ix:      0x{:02X}", core.ix);
        println!("iy:      0x{:02X}", core.iy);
        println!("ir:      0x{:02X}", core.ir);
        println!("infor:   {:?}", core.info);
        // Looking at a bare function pointer isn't very helpful.
        //println!("decoded: {:?}", core.decoded);

        // Very ugly memory dump code. Needs tidying up to be useful later.
        /*let mut result = Vec::new();
        let mut i = 0;
        
        while i < core.memory.len() {
            let current_value = core.memory[i];
            let mut count = 1;
    
            while i + count < core.memory.len() && core.memory[i + count] == current_value {
                count += 1;
            }
    
            if count > 1 {
                result.push(format!("0x{:02X}:{}", current_value, count));
            } else {
                result.push(format!("0x{:02X}", current_value));
            }
    
            i += count;
        }

        print!("memory:");
        for r in 0..result.len() {
            if result[r].contains(':') {
                println!("\n {},", result[r])
            } else {
                print!("{}, ", result[r])
            }
        }
        println!(); */
    }
}

// The load, fetch and decode functions are short, but are separated for clarity.
// Initializing the core and loading the binary data.
fn load(data: &Vec<u8>, start: &u16) -> Core {
    let mut core: Core = Core::new();

    core.sp = 0xFF; // Initialize stack pointer

    // Load the data into memory.
    let start_index: usize = *start as usize;
    let end_index: usize = start_index + data.len();

    if end_index > core.memory.len() { panic!("ROM data exceeds memory bounds!") }

    core.memory[start_index..end_index].copy_from_slice(&data);

    core
}

// Function for loading the next instruction from memory.
fn fetch(core: &mut Core) {
    core.ir = core.memory[core.pc as usize];
}

// Decoding the instruction using the prefix tree.
fn decode(core: &mut Core, prefix_tree: &Trie) {
    core.decoded = prefix_tree.get_function(core.ir);
    core.info = prefix_tree.get_instruction(core.ir);
}

// Parses function pointer from prefix tree and executes it.
fn execute(core: &mut Core) {
    match core.decoded {
        Some(func) => {
            func(core);
        }
        None => {
            panic!("Invalid opcode: {:02X}", core.ir)
        }
    }
}

fn set_pc(core: &mut Core, target: u16) -> &mut Core {
    let pcl: u8 = (target & 0xFF) as u8; // Lower byte
    let pch: u8 = (target >> 8) as u8; // Higher byte

    // Set initial reset vector then use it to set program counter.
    // A bit redundant, it's just symbolic I suppose for now.
    core.memory[0xFFFC..=0xFFFD].copy_from_slice(&[pcl, pch]);
    core.pc = ((core.memory[0xFFFD] as u16) << 8) | (core.memory[0xFFFC] as u16);

    core
}

fn parse_hex(start: &str) -> Result<u16, String> {
    if let Some(hex) = start.strip_prefix("0x") {
        u16::from_str_radix(hex, 16)
            .map_err(|e| format!("Invalid hex value: {e}"))
    } else {
        Err("Value must start with 0x".to_string())
    }
}

fn help_out(args: Option<&str>) {
    match args {
        Some("exec") | Some("EXEC") => {
            println!("exec <target>, EXEC <target> :");
            println!(" + Runs the passed binary starting at the given address.");
            println!(" + <target> must be a hexadecimal address starting with 0x.");
            println!("Examples: exec 0x200, exec 0x0200, exec 0x0");
        }
        Some("dump") | Some("DUMP") => {
            println!("dump <target>, DUMP <target>:");
            println!(" + Dumps the contents of memory at the target address.");
            println!(" + <target> must be a hexadecimal address starting with 0x.");
            println!(" + Lists of addresses, split by spaces, are supported.");
            println!("Examples: dump 0x200 0x300, dump 0x0200, dump 0x0 0x0200");
        }
        Some("clear") | Some("CLEAR") => {
            println!("clear, CLEAR :");
            println!("Clears the screen.")
        }
        Some("quit") | Some("QUIT") | Some("q") => {
            println!("quit, QUIT, q :");
            println!("Quits the program.")
        }
        Some(_) => {
            println!("Unrecognized argument {args:?}")
        }
        None => {
            println!("Welcome to a silly fake shell!");
            println!("Commands:");
            println!(" + exec, EXEC - Runs the passed binary starting at the given address.");
            println!(" + dump, DUMP - Will eventually dump memory or parts of it.");
            println!(" + clear, CLEAR - Clear the screen.");
            println!(" + quit, QUIT, q - Quit, pretty self explanatory.");
            println!(" + help, HELP, h - Prints this message.");
            println!("Run `help <COMMAND> for more information.`")
        }
    }
}

// Separated the main loop for clarity
fn main_loop(core: &mut Core, prefix_tree: &Trie) {
    let mut i = 1;

    // Starting to step through test binary to implement opcodes.
    // This is getting cumbersome. Need to implement stepping through loop now.
    loop {
        fetch(core);

        decode(core, prefix_tree);
        
        execute(core);

        print!("\x1B[11A");
        Core::core_dump(&core);
        println!("Iteration: {}", i);
        io::stdout().flush().unwrap();

        // Skipping over iterations I've looked at closely
        if core.stat & 0b00010000 != 0b00010000 {
            print!("Press Enter to step, or type 'q' to quit: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "q" {
                println!("Halting emulation.");
                break;
            }
        } else {
            println!();
            break;
        }

        i += 1;
    }
}

fn mem_dump(core: &mut Core, targets: &Vec<u16>) {
    println!(" Address │ Contents ");
    println!("─────────┼─────────");
    for target in targets {
        println!(" 0x{:04X}  │ 0x{:02X} ", target, core.memory[*target as usize])
    }
}

pub fn emulator(data: &Vec<u8>, start: &u16, path: &String, prefix_tree: &Trie) {
    let mut core: Core = load(data, start);

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "Running {} : 0x{:04X}",
        path,
        start,
    );
    println!("Welcome to the fake shell!");
    println!("Run `help` to see the commands!");

    // Stupid fake shell. I did like the idea, so I will expand on it.
    // Maybe move the emulation loop to another function for clarity as it expands later.
    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_vec = input.split_whitespace().collect::<Vec<_>>();

        if input_vec.len() == 0 { continue }

        match input_vec[0].trim() {
            "exec" | "EXEC" => {
                if input_vec.len() == 1 {
                    help_out(Some("exec"));
                    continue
                } else {
                    let target: Result<u16, String> = parse_hex(input_vec[1]);
                    match target {
                        Ok(val) => {
                            set_pc(&mut core, val);
                        }
                        Err(error) => {
                            println!("{error}");
                            continue;
                        }
                    }
                }

                print!("\x1B[2J\x1B[1;1H");

                main_loop(&mut core, prefix_tree);
            }

            "quit" | "QUIT" | "q" => {
                println!("Exiting...");
                break;
            }

            "dump" | "DUMP" => {
                if input_vec.len() == 1 {
                    help_out(Some("dump"));
                    continue
                } else {
                    let mut targets: Vec<u16> = Vec::new();

                    for i in 1..input_vec.len() {
                        let target: Result<u16, String> = parse_hex(input_vec[i]);
                        match target {
                            Ok(val) => {
                                targets.push(val);
                            }
                            Err(error) => {
                                println!("{error}");
                                continue;
                            }
                        }
                    }

                    mem_dump(&mut core, &targets);
                }
            }

            "clear" | "CLEAR" => { print!("\x1B[2J\x1B[1;1H"); }

            "help" | "HELP" | "h" => {
                if input_vec.len() == 1 { help_out(None); }
                else if input_vec.len() == 2 { help_out(Some(input_vec[1])); }
                else { println!("Please provide a maximum of one argument.") }
            }

            _ => { println!("Unrecognized input: {:?}", input_vec); }
        }
    }
}