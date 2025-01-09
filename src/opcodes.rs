use crate::system::Core;

pub fn adc(core: &mut Core) -> &mut Core { core }

pub fn and(core: &mut Core) -> &mut Core {
    match core.ir {
        0x29_u8 => { // AND IMM
            core.acc = core.acc & core.memory[core.pc as usize + 1];
            if core.acc == 0x00_u8 { core.stat = core.stat | 0b00000100 } // Set zero flag
            if ((core.acc >> 6) & 0b1) == 0b1 { core.stat = core.stat | 0b10000000 } // Set negative flag
            core.pc += 2;
        }
        0x25_u8 => {}
        0x35_u8 => {}
        0x2d_u8 => {}
        0x3d_u8 => {}
        0x39_u8 => {}
        0x21_u8 => {}
        0x31_u8 => {}
        _ => unreachable!()
    }

    core
} 

pub fn asl(core: &mut Core) -> &mut Core { core } 

pub fn bcc(core: &mut Core) -> &mut Core { core } 

pub fn bcs(core: &mut Core) -> &mut Core { core } 

pub fn beq(core: &mut Core) -> &mut Core { core } 

pub fn bmi(core: &mut Core) -> &mut Core { core } 

pub fn bne(core: &mut Core) -> &mut Core { core } 

pub fn bpl(core: &mut Core) -> &mut Core { core } 

pub fn bvc(core: &mut Core) -> &mut Core { core } 

pub fn bvs(core: &mut Core) -> &mut Core { core } 

pub fn bit(core: &mut Core) -> &mut Core { core } 

pub fn brk(core: &mut Core) -> &mut Core { core } 

pub fn clc(core: &mut Core) -> &mut Core { core } 

pub fn cld(core: &mut Core) -> &mut Core { core } 

pub fn cli(core: &mut Core) -> &mut Core { core } 

pub fn clv(core: &mut Core) -> &mut Core { core } 

pub fn nop(core: &mut Core) -> &mut Core { core } 

pub fn pha(core: &mut Core) -> &mut Core { core } 

pub fn pla(core: &mut Core) -> &mut Core { core } 

pub fn php(core: &mut Core) -> &mut Core { core } 

pub fn plp(core: &mut Core) -> &mut Core { core } 

pub fn rti(core: &mut Core) -> &mut Core { core } 

pub fn rts(core: &mut Core) -> &mut Core { core } 

pub fn sec(core: &mut Core) -> &mut Core { core } 

pub fn sed(core: &mut Core) -> &mut Core {
    core.stat = core.stat | 0b00010000;
    core.pc += 1;

    core
} 

pub fn sei(core: &mut Core) -> &mut Core { core } 

pub fn tax(core: &mut Core) -> &mut Core { core } 

pub fn txa(core: &mut Core) -> &mut Core { core } 

pub fn tay(core: &mut Core) -> &mut Core { core } 

pub fn tya(core: &mut Core) -> &mut Core { core } 

pub fn tsx(core: &mut Core) -> &mut Core { core } 

pub fn txs(core: &mut Core) -> &mut Core { core } 

pub fn cmp(core: &mut Core) -> &mut Core { core } 

pub fn cpx(core: &mut Core) -> &mut Core { core } 

pub fn cpy(core: &mut Core) -> &mut Core { core } 

pub fn dec(core: &mut Core) -> &mut Core { core } 

pub fn dex(core: &mut Core) -> &mut Core { core } 

pub fn dey(core: &mut Core) -> &mut Core { core } 

pub fn inx(core: &mut Core) -> &mut Core { core } 

pub fn iny(core: &mut Core) -> &mut Core { core } 

pub fn eor(core: &mut Core) -> &mut Core { core } 

pub fn inc(core: &mut Core) -> &mut Core { core } 

pub fn jmp(core: &mut Core) -> &mut Core { core } 

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
    match core.ir {
        0xa9_u8 => { // LDA IMM
            core.acc = core.memory[core.pc as usize + 1];
            if core.acc == 0x00_u8 { core.stat = core.stat | 0b00000100 } // Set zero flag
            if ((core.acc >> 6) & 0b1) == 0b1 { core.stat = core.stat | 0b10000000 } // Set negative flag
            core.pc += 2;
        }
        0xa5_u8 => { // LDA ZP
            core.acc = core.memory[core.memory[core.pc as usize + 1] as usize];
            if core.acc == 0x00_u8 { core.stat = core.stat | 0b00000100 } // Set zero flag
            if ((core.acc >> 6) & 0b1) == 0b1 { core.stat = core.stat | 0b10000000 } // Set negative flag
            core.pc += 2;
        }
        0xb5_u8 => {}
        0xad_u8 => {}
        0xbd_u8 => {}
        0xb9_u8 => {}
        0xa1_u8 => {}
        0xb1_u8 => {}
        _ => unreachable!()
    }

    core
} 

pub fn ldx(core: &mut Core) -> &mut Core { core } 

pub fn ldy(core: &mut Core) -> &mut Core {
    match core.ir {
        0xa0_u8 => { // LDY IMM
            core.iy = core.memory[core.pc as usize + 1];
            if core.iy == 0x00_u8 { core.stat = core.stat | 0b00000100 } // Set zero flag
            if ((core.iy >> 6) & 0b1) == 0b1 { core.stat = core.stat | 0b10000000 } // Set negative flag
            core.pc += 2;
        }
        0xa4_u8 => {}
        0xb4_u8 => {}
        0xac_u8 => {}
        0xbc_u8 => {}
        _ => unreachable!()
    }

    core
} 

pub fn lsr(core: &mut Core) -> &mut Core { core } 

pub fn ora(core: &mut Core) -> &mut Core {
    match core.ir {
        0x09_u8 => { // ORA IMM
            core.acc = core.acc | core.memory[core.pc as usize + 1];
            if core.acc == 0x00_u8 { core.stat = core.stat | 0b00000100 } // Set zero flag
            if ((core.acc >> 6) & 0b1) == 0b1 { core.stat = core.stat | 0b10000000 } // Set negative flag
            core.pc += 2;
        }
        0x05_u8 => {}
        0x15_u8 => {}
        0x0d_u8 => {}
        0x1d_u8 => {}
        0x19_u8 => {}
        0x01_u8 => {}
        0x11_u8 => {}
        _ => unreachable!()
    }

    core
} 

pub fn rol(core: &mut Core) -> &mut Core { core } 

pub fn ror(core: &mut Core) -> &mut Core { core } 

pub fn sbc(core: &mut Core) -> &mut Core { core } 

pub fn sta(core: &mut Core) -> &mut Core {
    match core.ir {
        0x85_u8 => { // STA ZP
            core.memory[core.memory[core.pc as usize + 1] as usize] = core.acc;
            core.pc += 2;
        }
        0x95_u8 => {}
        0x8d_u8 => {}
        0x9d_u8 => {}
        0x99_u8 => {}
        0x81_u8 => {}
        0x91_u8 => {}
        _ => unreachable!()
    }

    core
} 

pub fn stx(core: &mut Core) -> &mut Core {
    match core.ir {
        0x86_u8 => {}
        0x96_u8 => {}
        0x8e_u8 => {}
        _ => unreachable!()
    }

    core
} 

pub fn sty(core: &mut Core) -> &mut Core {
    match core.ir {
        0x84_u8 => { // STY ZP
            core.memory[core.memory[core.pc as usize + 1] as usize] = core.iy;
            core.pc += 2;
        }
        0x94_u8 => {}
        0x8c_u8 => {}
        _ => unreachable!()
    }

    core
} 

// I was going to write tests but I'm really unsure how to go about it it's very overwhelming.