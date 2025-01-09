use crate::trie::Trie;

pub struct Core {
    pub acc: u8, // 8-bit accumulator register
    pub stat: u8, // 7-bit status register, stored as u8
    // negative, overflow, break, decimal, interrupt disable, zero, carry, with final bit ignored
    pub pc: u16, // 16-bit program counter
    pub sp: u8, // 8-bit stack pointer
    pub ix: u8, // 8-bit index register
    pub iy: u8, // 8-bit index register
    pub ir: u8, // 8-bit instruction register
    pub decoded: Option<fn(&mut Core) -> &mut Core>, // Stores opcode funciton pointer
    // Note: This doesn't align with any particular systems, it is just enough to 
    // load specific 6502 test binaries.
    pub memory: [u8; 16384], // 16kb of memory
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
            memory: [0; 16384],
        }
    }

    // Function for dumping the state of the core.
    // No display built in to the 6502, therefore this will be useful
    // for debugging and testing later.
    pub fn core_dump(core: &Self) {
        println!("-->core dump<--");
        println!("acc:     0x{:02X}", core.acc);
        println!("stat:    0b{:08b}", core.stat);
        println!("pc:      0x{:04X}", core.pc);
        println!("sp:      0x{:02X}", core.sp);
        println!("ix:      0x{:02X}", core.ix);
        println!("iy:      0x{:02X}", core.iy);
        println!("ir:      0x{:02X}", core.ir);
        println!("decoded: {:?}", core.decoded);

        // Very ugly memory dump code. Needs tidying up to be useful later.
        let mut result = Vec::new();
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
        println!();
    }
}

// The load, fetch and decode functions are short, but are separated for clarity.
// Initializing the core and loading the binary data.
fn load(data: &Vec<u8>, start: &u16) -> Core {
    let mut core: Core = Core::new();

    core.memory[(*start as usize)..(*start as usize) + data.len()]
    .copy_from_slice(&data);

    core.pc = *start;

    core
}

// Function for loading the next instruction from memory.
fn fetch(core: &mut Core) {
    core.ir = core.memory[core.pc as usize];
}

// Decoding the instruction using the prefix tree.
fn decode(core: &mut Core, prefix_tree: &Trie) {
    core.decoded = prefix_tree.get_function(core.ir);
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

pub fn emulator(data: &Vec<u8>, start: &u16, prefix_tree: &Trie) {
    let mut core: Core = load(data, start);

    let mut i = 0;

    // Starting to step through test binary to implement opcodes.
    // This is getting cumbersome. Need to implement stepping through loop now.
    loop {
        fetch(&mut core);

        decode(&mut core, prefix_tree);
        
        execute(&mut core);

        i += 1;

        if i == 10 { break; }
    }

    println!();
    Core::core_dump(&core);
}