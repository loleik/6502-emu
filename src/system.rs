use crate::trie::Trie;

use std::io::{self, Write};

pub struct Core {
    pub acc: u8, // 8-bit accumulator register
    pub stat: u8, // 7-bit status register, stored as u8
    // negative, overflow, break, decimal, interrupt disable, zero, carry, with final bit ignored
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
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();


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
fn load(data: &Vec<u8>, start: &u16, exec: &u16) -> Core {
    let mut core: Core = Core::new();

    let pcl: u8 = (exec & 0xFF) as u8; // Lower byte
    let pch: u8 = (exec >> 8) as u8; // Higher byte

    core.sp = 0xFF; // Initialize stack pointer

    // Set initial reset vector then use it to set program counter.
    // A bit redundant, it's just symbolic I suppose for now.
    core.memory[0xFFFC..=0xFFFD].copy_from_slice(&[pcl, pch]);
    core.pc = ((core.memory[0xFFFD] as u16) << 8) | (core.memory[0xFFFC] as u16);

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
            panic!("Invalid opcode: {}", core.ir)
        }
    }
}

pub fn emulator(data: &Vec<u8>, start: &u16, exec: &u16, prefix_tree: &Trie) {
    let mut core: Core = load(data, start, exec);

    let mut i = 1;

    // Starting to step through test binary to implement opcodes.
    // This is getting cumbersome. Need to implement stepping through loop now.
    loop {
        fetch(&mut core);

        decode(&mut core, prefix_tree);
        
        execute(&mut core);

        Core::core_dump(&core);

        println!("Iteration: {}", i);

        // Skipping over iterations I've looked at closely
        if i >= 40 {
            print!("Press Enter to step, or type 'q' to quit: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "q" {
                println!("Halting emulation.");
                break;
            }
        }

        i += 1;
    }

    println!();
}