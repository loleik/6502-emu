pub struct Core {
    acc: u8, // 8-bit accumulator register
    stat: u8, // 7-bit status register, stored as u8
    pc: u16, // 16-bit program counter
    sp: u8, // 8-bit stack pointer
    ix: u8, // 8-bit index register
    iy: u8, // 8-bit index register
    ir: u8, // 8-bit instruction register
    // Note: This doesn't align with any particular systems, it is just enough to 
    // load specific 6502 test binaries.
    memory: [u8; 16384], // 16kb of memory
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
            memory: [0; 16384],
        }
    }

    // Function for dumping the state of the core.
    // No display built in to the 6502, therefore this will be useful
    // for debugging and testing later.
    pub fn core_dump(core: &Self) {
        println!("-->core dump<--");
        println!("acc:    0x{:02X}", core.acc);
        println!("stat:   0x{:02X}", core.stat);
        println!("pc:     0x{:04X}", core.pc);
        println!("sp:     0x{:02X}", core.sp);
        println!("ix:     0x{:02X}", core.ix);
        println!("iy:     0x{:02X}", core.iy);
        println!("ir:     0x{:02X}", core.ir);

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

// The load and fetch functions are short, but are separated for clarity.
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

pub fn emulator(data: &Vec<u8>, start: &u16) -> std::io::Result<()> {
    let mut core: Core = load(data, start);

    fetch(&mut core);

    Core::core_dump(&core);

    Ok(())
}