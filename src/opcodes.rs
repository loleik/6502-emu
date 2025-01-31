use std::result;

use crate::system::Core;
use crate::addressing::{zero_page, zero_page_x, zero_page_y,
                        absolute, absolute_x, absolute_y,
                        relative};

enum Value {
    U8(u8),
    U16(u16),
}

impl Value {
    fn get_u8(&self) -> u8 {
        match self {
            Value::U8(value) => *value,
            _ => unreachable!(),
        }
    }
}

pub fn adc(core: &mut Core) -> &mut Core {
    // Check for the decimal mode flag, as it means we have to work with binary coded decimal.
    let _decimal: bool = if (core.stat >> 3) & 0b1 == 0 { false } else { true };

    let value: u8;
    let inc: u16;

    match core.ir {
        0x69_u8 => { // ADC IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0x65_u8 => { // ADC ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0x75_u8 => { // ADC ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0x6d_u8 => { // ADC ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0x7d_u8 => { // ADC ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0x79_u8 => { // ADC ABSY
            let absx: u16 = absolute_y(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        //0x61_u8 => {}
        //0x71_u8 => {}
        _ => unreachable!("{:?}", core.info)
    }

    let borrow: u8 = if core.stat & 0b1 != 0 { 1 } else { 0 }; // Equal to carry bit

    // Calculate A + M + Carry, zero extending all values:
    let result: i16 = (core.acc as i16) + (value as i16) + (borrow as i16);

    let acc_sign: bool = (core.acc >> 7) & 0b1 != 0; // Sign used for overflow check before addition.

    core.acc = result as u8; // Clamp to 8 bits and store result

    if result > 0xff { core.stat |= 0b00000001 } // set carry flag
    else { core.stat &= !0b00000001 } // clear carry flag

    if core.acc == 0 { core.stat |= 0b00000010 } // set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    // Overflow flag logic:
    let mem_sign: bool = (value >> 7) & 0b1 != 0;
    let res_sign: bool = (core.acc >> 7) & 0b1 != 0;

    if acc_sign == mem_sign && acc_sign != res_sign { core.stat |= 0b01000000 } // set overflow flag
    else { core.stat &= !0b01000000 } // clear overflow flag    

    core.pc += inc;

    core
}

pub fn and(core: &mut Core) -> &mut Core {
    let value: u8;
    let inc: u16;

    match core.ir {
        0x29_u8 => { // AND IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0x25_u8 => { // AND ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0x35_u8 => { // AND ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0x2d_u8 => { // AND ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0x3d_u8 => { // AND ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0x39_u8 => { // AND ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        //0x21_u8 => {}
        //0x31_u8 => {}
        _ => unreachable!("{:?}", core.info)
    }

    core.acc &= value;

    if core.acc == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    core.pc += inc;

    core
} 

pub fn asl(core: &mut Core) -> &mut Core {
    let new_carry: bool;
    let inc: u16;
    let result: u8;

    match core.ir {
        0xa0_u8 => { // ASL ACC
            new_carry = if (core.acc >> 7) & 0b1 != 0 { true } else { false };
            core.acc <<= 1;
            result = core.acc;
            inc = 1;
        }
        0x06_u8 => { // ASL ZP
            let zp: u8 = zero_page(core);
            new_carry = if (core.memory[zp as usize] >> 7) & 0b1 != 0 { true } else { false };
            core.memory[zp as usize] <<= 1;
            result = core.memory[zp as usize];
            inc = 2;
        }
        0x16_u8 => { // ASL ZPX
            let zpx: u8 = zero_page_x(core);
            new_carry = if (core.memory[zpx as usize] >> 7) & 0b1 != 0 { true } else { false };
            core.memory[zpx as usize] <<= 1;
            result = core.memory[zpx as usize];
            inc = 2;
        }
        0x0e_u8 => { // ASL ABS
            let abs: u16 = absolute(core);
            new_carry = if (core.memory[abs as usize] >> 7) & 0b1 != 0 { true } else { false };
            core.memory[abs as usize] <<= 1;
            result = core.memory[abs as usize];
            inc = 3;
        }
        0x1e_u8 => { // ASL ABSX
            let absx: u16 = absolute_x(core);
            new_carry = if (core.memory[absx as usize] >> 7) & 0b1 != 0 { true } else { false };
            core.memory[absx as usize] <<= 1;
            result = core.memory[absx as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    match result {
        0x00 => { core.stat |= 0b00000010 } // Set zero flag
        _ => { core.stat &= !0b00000010 } // Clear zero flag
    }

    if (result >> 7) & 0b1 == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    match new_carry {
        true => { core.stat |= 0b00000001 } // Set carry flag
        false => { core.stat &= !0b00000001 } // Clear carry flag
    }
    
    core.pc += inc;

    core
} 

pub fn bcc(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if core.stat & 0b1 == 0 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }

    core.pc += 2;

    core
} 

pub fn bcs(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if core.stat & 0b1 == 1 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn beq(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if (core.stat >> 1) & 0b1 == 1 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn bmi(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if (core.stat >> 7) & 0b1 == 1 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn bne(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if (core.stat >> 1) & 0b1 == 0 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn bpl(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if (core.stat >> 7) & 0b1 == 0 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn bvc(core: &mut Core) -> &mut Core {
    // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if (core.stat >> 6) & 0b1 == 0 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn bvs(core: &mut Core) -> &mut Core {
        // Grab signed offset safely casted as a signed 32 bit integer to handle overflow safely
    // Add this value to program counter casted as an i32 safely to handle overflow
    if (core.stat >> 6) & 0b1 == 1 {
        core.pc = ((core.pc as i32) + (relative(core) as i32)) as u16;
    }
    
    core.pc += 2;

    core
} 

pub fn bit(core: &mut Core) -> &mut Core { core } 

pub fn brk(core: &mut Core) -> &mut Core {
    // Grab the full stack pointer address.
    let stack_address: usize = 0x0100 | (core.sp as u16) as usize;

    // Get the bytes of the return address, PC + 2.
    let spcl: u8 = (core.pc + 2 & 0xFF00) as u8;
    let spch: u8 = (core.pc + 2 >> 8) as u8;

    // Set the break flag.
    core.stat = core.stat | 0b00010000;

    // Store the status register and return address in the stack.
    core.memory[stack_address - 2..=stack_address]
        .copy_from_slice(&[core.stat, spcl, spch]);

    // Descend stack pointer.
    core.sp -= 3;

    // Set PC to interrupt vector. Just symbolic for now.
    let pcl: u16 = core.memory[0xfffe] as u16;
    let pch: u16 = (core.memory[0xffff] as u16) << 8;

    core.pc = pch | pcl;

    core
} 

pub fn clc(core: &mut Core) -> &mut Core {
    core.stat &= !0b00000001; // Clear carry flag

    core.pc += 1;

    core
} 

pub fn cld(core: &mut Core) -> &mut Core {
    core.stat = core.stat & 0b11110111; // Clear decimal flag

    core.pc += 1;

    core
} 

pub fn cli(core: &mut Core) -> &mut Core {
    core.stat = core.stat & 0b11111011; // Clear interrupt flag

    core.pc += 1;

    core
} 

pub fn clv(core: &mut Core) -> &mut Core {
    core.stat = core.stat & 0b10111111; // Clear overflow flag

    core.pc += 1;

    core
} 

pub fn nop(core: &mut Core) -> &mut Core {
    core.pc += 1;

    core
} 

pub fn pha(core: &mut Core) -> &mut Core {
    // Push accumulator to stack.
    core.memory[(0x100 | core.sp as u16) as usize] = core.acc;

    core.sp -= 1; // Descend stack pointer

    core.pc += 1;

    core
} 

pub fn pla(core: &mut Core) -> &mut Core {
    // Set accumulator to value from the stack
    core.acc = core.memory[(0x100 | (core.sp as u16 + 1)) as usize];

    // Wipe the value from the stack
    core.memory[(0x100 | (core.sp as u16 + 1)) as usize] = 0x00;

    // Increment stack pointer
    core.sp += 1;

    if core.acc == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    core.pc += 1;

    core
} 

pub fn php(core: &mut Core) -> &mut Core {
    // Set break flag and push status to stack.
    core.memory[(0x100 | core.sp as u16) as usize] = core.stat | 0b00010000;

    core.sp -= 1; // Descend stack pointer

    core.pc += 1;
    
    core
} 

pub fn plp(core: &mut Core) -> &mut Core {
    // Set status to value from the stack
    core.stat = core.memory[(0x100 | (core.sp as u16 + 1)) as usize];

    // Wipe the value from the stack
    core.memory[(0x100 | (core.sp as u16 + 1)) as usize] = 0x00;

    // Increment stack pointer
    core.sp += 1;

    core.pc += 1;

    core
} 

pub fn rti(core: &mut Core) -> &mut Core { core } 

pub fn rts(core: &mut Core) -> &mut Core {
    // Get the new PC value from the stack.
    let stack_address: usize = 0x0100 | (core.sp as u16) as usize;

    let pcl: u16 = core.memory[stack_address + 1] as u16;
    let pch: u16 = (core.memory[stack_address + 2] as u16) << 8;

    // Set the new PC value.
    core.pc = pch | pcl;

    // Adjust stack pointer to ascend with the stack.
    core.sp += 2;

    core.pc += 1;

    core
} 

pub fn sec(core: &mut Core) -> &mut Core {
    // Set carry flag
    core.stat |= 0b00000001;
    core.pc += 1;

    core
} 

pub fn sed(core: &mut Core) -> &mut Core {
    core.stat |= 0b00001000;
    core.pc += 1;

    core
} 

pub fn sei(core: &mut Core) -> &mut Core {
    core.stat |= 0b00000100; // Set interrupt flag
    core.pc += 1;

    core
} 

pub fn tax(core: &mut Core) -> &mut Core {
    core.ix = core.acc;

    if core.ix == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.ix >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn txa(core: &mut Core) -> &mut Core {
    core.acc = core.ix;

    if core.acc == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn tay(core: &mut Core) -> &mut Core {
    core.iy = core.acc;

    if core.iy == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.iy >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn tya(core: &mut Core) -> &mut Core {
    core.acc = core.iy;

    if core.acc == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn tsx(core: &mut Core) -> &mut Core {
    core.ix = core.sp;

    if core.ix == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.ix >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn txs(core: &mut Core) -> &mut Core {
    core.sp = core.ix;

    core.pc += 1;

    core
} 

pub fn cmp(core: &mut Core) -> &mut Core {
    // Check for the decimal mode flag, as it means we have to work with binary coded decimal.
    let _decimal: bool = if (core.stat >> 3) & 0b1 == 0 { false } else { true };

    let value: u8;
    let inc: u16;

    match core.ir {
        0xC9 => { // CMP IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xC5 => { // CMP ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xD5 => { // CMP ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0xCD => { // CMP ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0xDD => { // CMP ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0xD9 => { // CMP ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        //0xC1 => {}
        //0xD1 => {}
        _ => unreachable!("{:?}", core.info)
    }

    // Calculate A - M, zero extending both values:
    let result: i16 = (core.acc as i16) - (value as i16);

    // Check the result and set flags:
    if core.acc >= value { core.stat |= 0b00000001 } // Carry flag
    else { core.stat &= !0b00000001 } // Clear carry flag

    if (result & 0xFF) == 0 { core.stat |= 0b00000010 } // Zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if (result & 0x80) != 0 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += inc;

    core
} 

pub fn cpx(core: &mut Core) -> &mut Core {
    // Check for the decimal mode flag, as it means we have to work with binary coded decimal.
    let _decimal: bool = if (core.stat >> 3) & 0b1 == 0 { false } else { true };

    let value: u8;
    let inc: u16;

    match core.ir {
        0xe0 => { // CPX IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xe4 => { // CPX ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xec => { // CPX ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    // Calculate Y - M, zero extending both values:
    let result: i16 = (core.ix as i16) - (value as i16);

    // Check the result and set flags:
    if result >= 0 { core.stat |= 0b00000001 } // Carry flag
    else { core.stat &= !0b00000001 } // Clear carry flag

    if (result & 0xFF) == 0 { core.stat |= 0b00000010 } // Zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if (result & 0x80) != 0 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += inc;

    core
} 

pub fn cpy(core: &mut Core) -> &mut Core {
    // Check for the decimal mode flag, as it means we have to work with binary coded decimal.
    let _decimal: bool = if (core.stat >> 3) & 0b1 == 0 { false } else { true };

    let value: u8;
    let inc: u16;

    match core.ir {
        0xC0 => { // CPY IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xC4 => { // CPY ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xCC => { // CPY ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    // Calculate Y - M, zero extending both values:
    let result: i16 = (core.iy as i16) - (value as i16);

    // Check the result and set flags:
    if result >= 0 { core.stat |= 0b00000001 } // Carry flag
    else { core.stat &= !0b00000001 } // Clear carry flag

    if (result & 0xFF) == 0 { core.stat |= 0b00000010 } // Zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if (result & 0x80) != 0 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += inc;

    core
} 

pub fn dec(core: &mut Core) -> &mut Core {
    match core.ir {
        //0xc6 => {}
        //0xd6 => {}
        //0xce => {}
        //0xde => {}
        _ => unreachable!("{:?}", core.info)
    }

    //core
} 

pub fn dex(core: &mut Core) -> &mut Core {
    core.ix = core.ix.wrapping_sub(1);

    if core.ix == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.ix >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn dey(core: &mut Core) -> &mut Core {
    core.iy = core.iy.wrapping_sub(1);

    if core.iy == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.iy >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn inx(core: &mut Core) -> &mut Core {
    core.ix = core.ix.wrapping_add(1);

    if core.ix == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.ix >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn iny(core: &mut Core) -> &mut Core {
    core.iy = core.iy.wrapping_add(1);

    if core.iy == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.iy >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += 1;

    core
} 

pub fn eor(core: &mut Core) -> &mut Core {
    let value: u8;
    let inc: u16;
    
    match core.ir {
        0x49 => { // EOR IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0x45 => { // EOR ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0x55 => { // EOR ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0x4D => { // EOR ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0x5D => { // EOR ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0x59 => { // EOR ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        //0x41 => {}
        //0x51 => {}
        _ => unreachable!("{:?}", core.info)
    }

    // Calculate A ^ M
    core.acc ^= value;

    if core.acc == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += inc;

    core
} 

pub fn inc(core: &mut Core) -> &mut Core {
    let address: Value;
    let inc: u16;
    
    match core.ir {
        0xE6 => { // INC ZP
            address = Value::U8(zero_page(core));
            inc = 2;
        }
        0xF6 => { // INC ZPX
            address = Value::U8(zero_page_x(core));
            inc = 2;
        }
        0xEE => { // INC ABS
            address = Value::U16(absolute(core));
            inc = 3;
        }
        0xFE => { // INC ABSX
            address = Value::U16(absolute_x(core));
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    match address {
        Value::U8(val) => {
            core.memory[val as usize] = core.memory[val as usize].wrapping_add(1);
        }
        Value::U16(val) => {
            core.memory[val as usize] = core.memory[val as usize].wrapping_add(1);
        }
    }

    if core.memory[address.get_u8() as usize] == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // Clear zero flag
    
    if ((core.memory[address.get_u8() as usize] >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // Clear negative flag

    core.pc += inc;

    core
} 

// NOTE refactor to use Value struct
pub fn jmp(core: &mut Core) -> &mut Core {
    match core.ir {
        0x4C => { // JMP ABS
            let address: u16 = absolute(core);

            core.pc = address;
        }
        0x6C => {}
        _ => unreachable!("{:?}", core.info)
    }

    core
} 

pub fn jsr(core: &mut Core) -> &mut Core {
    // The return address - 1 due to how RTS works.
    let spl: u8 = ((core.pc + 2) & 0xFF) as u8; // Lower byte
    let sph: u8 = ((core.pc + 2) >> 8) as u8; // Higher byte

    // Subroutine address.
    let pcl: u8 = core.memory[core.pc as usize + 1]; // Lower byte
    let pch: u8 = core.memory[core.pc as usize + 2]; // Higher byte

    // Store the return address - 1 in memory. 
    let stack_address: usize = 0x0100 | (core.sp as u16) as usize;
    core.memory[(stack_address - 1)..=(stack_address)]
        .copy_from_slice(&[spl, sph]);

    // Adjust stack pointer to descend with the stack.
    core.sp -= 2;

    // Adjust program counter.
    core.pc = ((pch as u16) << 8) | (pcl as u16);

    core
} 

pub fn lda(core: &mut Core) -> &mut Core {
    let value: u8;
    let inc: u16;

    match core.ir {
        0xa9_u8 => { // LDA IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xa5_u8 => { // LDA ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xb5_u8 => { // LDA ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0xad_u8 => { // LDA ABS
            let abs: u16 =  absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0xbd_u8 => { // LDA ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0xb9_u8 => { // LDA ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        //0xa1_u8 => {}
        //0xb1_u8 => {}
        _ => unreachable!("{:?}", core.info)
    }

    core.acc = value;

    if core.acc == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    core.pc += inc;

    core
} 

pub fn ldx(core: &mut Core) -> &mut Core {
    let value: u8;
    let inc: u16;

    match core.ir {
        0xA2_u8 => { // LDX IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xA6_u8 => { // LDX ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xB6_u8 => { // LDX ZPY
            let zpy: u8 = zero_page_y(core);
            value = core.memory[zpy as usize];
            inc = 2;
        }
        0xAE_u8 => { // LDX ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0xBE_u8 => { // LDX ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    core.ix = value;

    if core.ix == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.ix >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    core.pc += inc;

    core
} 

pub fn ldy(core: &mut Core) -> &mut Core {
    let value: u8;
    let inc: u16;

    match core.ir {
        0xa0_u8 => { // LDY IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xa4_u8 => { // LDY ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xb4_u8 => { // LDY ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0xac_u8 => { // LDY ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0xbc_u8 => { // LDY ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    core.iy = value;

    if core.iy == 0 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.iy >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    core.pc += inc;

    core
} 

pub fn lsr(core: &mut Core) -> &mut Core {
    let new_carry: bool;
    let result: u8;
    let inc: u16;

    match core.ir {
        0x4a => { // LSR ACC
            new_carry = if (core.acc & 0b1) != 0 { true } else { false };

            core.acc >>= 1;

            result = core.acc;
            inc = 1;
        }
        0x46 => { // LSR ZP
            let zp: u8 = zero_page(core);

            new_carry = if (core.memory[zp as usize] & 0b1) != 0 { true } else { false };

            core.memory[zp as usize] >>= 1;

            result = core.memory[zp as usize];
            inc = 2;
        }
        0x56 => { // LSR ZPX
            let zpx: u8 = zero_page_x(core);

            new_carry = if (core.memory[zpx as usize] & 0b1) != 0 { true } else { false };

            core.memory[zpx as usize] >>= 1;

            result = core.memory[zpx as usize];
            inc = 2;
        }
        0x4e => { // LSR ABS
            let abs: u16 = absolute(core);

            new_carry = if (core.memory[abs as usize] & 0b1) != 0 { true } else { false };

            core.memory[abs as usize] >>= 1;

            result = core.memory[abs as usize];
            inc = 3;
        }
        0x5e => { // LSR ABSX
            let absx: u16 = absolute_x(core);

            new_carry = if (core.memory[absx as usize] & 0b1) != 0 { true } else { false };

            core.memory[absx as usize] >>= 1;

            result = core.memory[absx as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    if new_carry { core.stat |= 0b00000001 } // Set carry flag
    else { core.stat &= !0b00000001 } // clear carry flag

    if result == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    core.stat &= !0b10000000; // clear negative flag

    core.pc += inc;

    core
} 

pub fn ora(core: &mut Core) -> &mut Core {
    let value: u8;
    let inc: u16;

    match core.ir {
        0x09_u8 => { // ORA IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0x05_u8 => { // ORA ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0x15_u8 => { // ORA ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0x0d_u8 => { // ORA ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0x1d_u8 => { // ORA ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0x19_u8 => { // ORA ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        //0x01_u8 => {}
        //0x11_u8 => {}
        _ => unreachable!("{:?}", core.info)
    }

    core.acc |= value;

    if core.acc == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    core.pc += inc;

    core
} 

pub fn rol(core: &mut Core) -> &mut Core {
    let old_carry: u8 = core.stat & 0b00000001;

    let new_carry: bool;
    let result: u8;
    let inc: u16;

    match core.ir {
        0x2a => { // ROL ACC
            new_carry = if (core.acc >> 7) & 0b1 != 0 { true } else { false };

            core.acc <<= 1;

            core.acc |= old_carry;

            result = core.acc;
            inc = 1;
        }
        0x26 => { // ROL ZP
            let zp: u8 = zero_page(core);

            new_carry = if (core.memory[zp as usize] >> 7) & 0b1 != 0 { true } else { false };

            core.memory[zp as usize] <<= 1;

            core.memory[zp as usize] |= old_carry;

            result = core.memory[zp as usize];
            inc = 2;
        }
        0x36 => {
            let zpx: u8 = zero_page_x(core);

            new_carry = if (core.memory[zpx as usize] >> 7) & 0b1 != 0 { true } else { false };

            core.memory[zpx as usize] <<= 1;

            core.memory[zpx as usize] |= old_carry;

            result = core.memory[zpx as usize];
            inc = 2;
        }
        0x2e => { // ROL ABS
            let abs: u16 = absolute(core);

            new_carry = if (core.memory[abs as usize] >> 7) & 0b1 != 0 { true } else { false };

            core.memory[abs as usize] <<= 1;

            core.memory[abs as usize] |= old_carry;

            result = core.memory[abs as usize];
            inc = 3;
        }
        0x3e => { // ROL ABSX
            let absx: u16 = absolute_x(core);

            new_carry = if (core.memory[absx as usize] >> 7) & 0b1 != 0 { true } else { false };

            core.memory[absx as usize] <<= 1;

            core.memory[absx as usize] |= old_carry;

            result = core.memory[absx as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    if result == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((result >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    if new_carry { core.stat |= 0b00000001 } // Set carry flag
    else { core.stat &= !0b00000001 } // clear carry flag

    core.pc += inc;

    core
} 

pub fn ror(core: &mut Core) -> &mut Core {
    let old_carry: u8 = core.stat & 0b00000001;

    let new_carry: bool;
    let result: u8;
    let inc: u16;

    match core.ir {
        0x6a => { // ROR ACC
            new_carry = if (core.acc & 0b1) != 0 { true } else { false };

            core.acc >>= 1;

            core.acc |= old_carry << 7;

            result = core.acc;
            inc = 1;
        }
        0x66 => { // ROR ZP
            let zp: u8 = zero_page(core);

            new_carry = if (core.memory[zp as usize] & 0b1) != 0 { true } else { false };

            core.memory[zp as usize] >>= 1;

            core.memory[zp as usize] |= old_carry << 7;

            result = core.memory[zp as usize];
            inc = 2;
        }
        0x76 => { // ROR ZPX   
            let zpx: u8 = zero_page_x(core);

            new_carry = if (core.memory[zpx as usize] & 0b1) != 0 { true } else { false };

            core.memory[zpx as usize] >>= 1;

            core.memory[zpx as usize] |= old_carry << 7;

            result = core.memory[zpx as usize];
            inc = 2;
        }
        0x7e => { // ROR ABSX
            let abs: u16 = absolute(core);

            new_carry = if (core.memory[abs as usize] & 0b1) != 0 { true } else { false };

            core.memory[abs as usize] >>= 1;

            core.memory[abs as usize] |= old_carry << 7;

            result = core.memory[abs as usize];
            inc = 3;
        }
        0x6e => { // ROR ABSX
            let absx: u16 = absolute_x(core);

            new_carry = if (core.memory[absx as usize] & 0b1) != 0 { true } else { false };

            core.memory[absx as usize] >>= 1;

            core.memory[absx as usize] |= old_carry << 7;

            result = core.memory[absx as usize];
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    if result == 0x00_u8 { core.stat |= 0b00000010 } // Set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((result >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // Set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    if new_carry { core.stat |= 0b00000001 } // Set carry flag
    else { core.stat &= !0b00000001 } // clear carry flag

    core.pc += inc;

    core
} 

pub fn sbc(core: &mut Core) -> &mut Core {
    // Check for the decimal mode flag, as it means we have to work with binary coded decimal.
    let _decimal: bool = if (core.stat >> 3) & 0b1 == 0 { false } else { true };

    let value: u8;
    let inc: u16;
    
    match core.ir {
        0xE9_u8 => { // SBC IMM
            value = core.memory[core.pc as usize + 1];
            inc = 2;
        }
        0xE5_u8 => { // SBC ZP
            let zp: u8 = zero_page(core);
            value = core.memory[zp as usize];
            inc = 2;
        }
        0xF5_u8 => { // SBC ZPX
            let zpx: u8 = zero_page_x(core);
            value = core.memory[zpx as usize];
            inc = 2;
        }
        0xED_u8 => { // SBC ABS
            let abs: u16 = absolute(core);
            value = core.memory[abs as usize];
            inc = 3;
        }
        0xFD_u8 => { // SBC ABSX
            let absx: u16 = absolute_x(core);
            value = core.memory[absx as usize];
            inc = 3;
        }
        0xF9_u8 => { // SBC ABSY
            let absy: u16 = absolute_y(core);
            value = core.memory[absy as usize];
            inc = 3;
        }
        //0xE1_u8 => {}
        //0xF1_u8 => {}
        _ => unreachable!("{:?}", core.info)
    }
    
    let acc_sign: bool = (core.acc >> 7) & 0b1 != 0; // Sign used for overflow check.

    let borrow: u8 = if core.stat & 0b1 == 0 { 1 } else { 0 }; // inverse of carry flag
            
    // Calculate A - M - Carry, zero extending all values:
    let result: i16 = (core.acc as i16) - (value as i16) - (borrow as i16);

    core.acc = result as u8; // Clamp to 8 bits and store result

    if result >= 0 { core.stat |= 0b00000001 } // set carry flag
    else { core.stat &= !0b00000001 } // clear carry flag

    if core.acc == 0 { core.stat |= 0b00000010 } // set zero flag
    else { core.stat &= !0b00000010 } // clear zero flag

    if ((core.acc >> 7) & 0b1) == 0b1 { core.stat |= 0b10000000 } // set negative flag
    else { core.stat &= !0b10000000 } // clear negative flag

    // Overflow flag logic:
    let mem_sign: bool = (value >> 7) & 0b1 != 0;
    let res_sign: bool = (core.acc >> 7) & 0b1 != 0;

    if acc_sign != mem_sign && acc_sign != res_sign { core.stat |= 0b01000000 } // set overflow flag
    else { core.stat &= !0b01000000 } // clear overflow flag

    core.pc += inc;

    core
} 

pub fn sta(core: &mut Core) -> &mut Core {
    let address: Value;
    let inc: u16;

    match core.ir {
        0x85_u8 => { // STA ZP
            address = Value::U8(zero_page(core));
            inc = 2;
        }
        0x95_u8 => { // STA ZPX
            address = Value::U8(zero_page_x(core));
            inc = 2;
        }
        0x8d_u8 => { // STA ABS
            address = Value::U16(absolute(core));
            inc = 3;
        }
        0x9d_u8 => { // STA ABSX
            address = Value::U16(absolute_x(core));
            inc = 3;
        }
        0x99_u8 => { // STA ABSY
            address = Value::U16(absolute_y(core));
            inc = 3;
        }
        //0x81_u8 => {}
        //0x91_u8 => {}
        _ => unreachable!("{:?}", core.info)
    }

    match address {
        Value::U8(addr) => core.memory[addr as usize] = core.acc,
        Value::U16(addr) => core.memory[addr as usize] = core.acc,
    }

    core.pc += inc;

    core
} 

pub fn stx(core: &mut Core) -> &mut Core {
    let address: Value;
    let inc: u16;

    match core.ir {
        0x86_u8 => { // STY ZP
            address = Value::U8(zero_page(core));
            inc = 2;
        }
        0x96_u8 => { // STY ZPX
            address = Value::U8(zero_page_y(core));
            inc = 2;
        }
        0x8e_u8 => { // STY ABS
            address = Value::U16(absolute(core));
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    match address {
        Value::U8(addr) => core.memory[addr as usize] = core.ix,
        Value::U16(addr) => core.memory[addr as usize] = core.ix,
    }

    core.pc += inc;

    core
} 

pub fn sty(core: &mut Core) -> &mut Core {
    let address: Value;
    let inc: u16;

    match core.ir {
        0x84_u8 => { // STY ZP
            address = Value::U8(zero_page(core));
            inc = 2;
        }
        0x94_u8 => { // STY ZPX
            address = Value::U8(zero_page_x(core));
            inc = 2;
        }
        0x8c_u8 => { // STY ABS
            address = Value::U16(absolute(core));
            inc = 3;
        }
        _ => unreachable!("{:?}", core.info)
    }

    match address {
        Value::U8(addr) => core.memory[addr as usize] = core.iy,
        Value::U16(addr) => core.memory[addr as usize] = core.iy,
    }

    core.pc += inc;

    core
} 

// I was going to write tests but I'm really unsure how to go about it it's very overwhelming.