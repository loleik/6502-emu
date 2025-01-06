pub struct Core {
    acc: u8, // 8-bit accumulator register
    stat: u8, // 7-bit status register, stored as u8
    pc: u16, // 16-bit program counter
    sp: u8, // 8-bit stack pointer
    ix: u8, // 8-bit index register
    iy: u8, // 8-bit index register
    // Note: This doesn't align with any particular systems, it is just enough to 
    // load specific 6502 test binaries.
    memory: [u8; 16384], // 16kb of memory.
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
            memory: [0; 16384],
        }
    }

    pub fn dump_core(core: Self) {
        println!("-->core dump<--");
        println!("acc:    {}", core.acc);
        println!("stat:   {}", core.stat);
        println!("pc:     {}", core.pc);
        println!("sp:     {}", core.sp);
        println!("ix:     {}", core.ix);
        println!("iy:     {}", core.iy);

        let mut result = Vec::new();
        let mut i = 0;
    
        while i < core.memory.len() {
            let current_value = core.memory[i];
            let mut count = 1;
    
            while i + count < core.memory.len() && core.memory[i + count] == current_value {
                count += 1;
            }
    
            if count > 1 {
                result.push(format!("{}:{}", current_value, count));
            } else {
                result.push(format!("{}", current_value));
            }
    
            i += count;
        }

        println!("memory: {}", result.join(", "));
    }
}

pub fn emulator() -> std::io::Result<()> {
    let mut core: Core = Core::new();

    Core::dump_core(core);

    Ok(())
}