use crate::system::Core;

pub fn adc(core: &mut Core) -> &mut Core { core }

pub fn and(core: &mut Core) -> &mut Core {
    match core.ir {
        0x29_u8 => {}
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

pub fn sed(core: &mut Core) -> &mut Core { core } 

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

pub fn jsr(core: &mut Core) -> &mut Core { core } 

pub fn lda(core: &mut Core) -> &mut Core { core } 

pub fn ldx(core: &mut Core) -> &mut Core { core } 

pub fn ldy(core: &mut Core) -> &mut Core {
    match core.ir {
        // LDY IMM
        0xa0_u8 => {
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

pub fn ora(core: &mut Core) -> &mut Core { core } 

pub fn rol(core: &mut Core) -> &mut Core { core } 

pub fn ror(core: &mut Core) -> &mut Core { core } 

pub fn sbc(core: &mut Core) -> &mut Core { core } 

pub fn sta(core: &mut Core) -> &mut Core { core } 

pub fn stx(core: &mut Core) -> &mut Core { core } 

pub fn sty(core: &mut Core) -> &mut Core {
    match core.ir {
        0x86_u8 => {}
        0x96_u8 => {}
        0x8e_u8 => {}
        0x84_u8 => {
            println!("0x84 : STY ZP");
            core.pc += 2;
        }
        0x94_u8 => {}
        0x8c_u8 => {}
        _ => unreachable!()
    }

    core
} 

// I was going to write tests but I'm really unsure how to go about it it's very overwhelming.