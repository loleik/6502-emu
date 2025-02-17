use crate::{opcodes::*, system::Core};

use std::collections::HashMap;

/* 
    The general idea of this implementation comes from this post:
    https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m

    It works as a prefix lookup tree, where we go through instructions bit by
    bit in order to find what opcode the instruction refers to, along with the
    addressing mode.

    Another, possibly more concise idea that I considered, was to have two 
    separate lookup arrays. One of which contains opcodes, and function calls 
    which take the addressing mode and data as input. The other would contain
    the addressing modes.
*/

/*
    This is the prefix tree for the disassembler. Right now I feel like there
    could need to be a separate version for the actual emulation, but I could
    also add another variable to the struct for storing function calls. I'll 
    deal with that later though.
*/

// Node struct
#[derive(Default, Debug)]
pub struct Node {
    is_prefix: bool,
    children: HashMap<u8, Node>,
    instruction: Option<String>,
    function: Option<fn(&mut Core) -> &mut Core>
}

// Trie struct
#[derive(Default, Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    // Create a new trie.
    pub fn new() -> Self {
        Trie { root: Node::default() }
    }

    // Inserting a new entry into a trie.
    pub fn insert(
        &mut self, 
        instruction: &u8, 
        info: String,
        function: Option<fn(&mut Core) -> &mut Core>
    ) {
        let mut current_node = &mut self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            current_node = current_node.children.entry(bit).or_default();
        }

        current_node.is_prefix = true;
        current_node.instruction = Some(info);
        current_node.function = function;
    }

    // Search a trie for an entry.
    pub fn contains(&self, instruction: u8) -> bool {
        let mut current_node = &self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            match current_node.children.get(&bit) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.is_prefix
    }

    // Grab instruction information for a trie entry.
    pub fn get_instruction(&self, instruction: u8) -> Option<String> {
        let mut current_node = &self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            match current_node.children.get(&bit) {
                Some(node) => current_node = node,
                None => return None,
            }
        }

        current_node.instruction.clone()
    }

    pub fn get_function(&self, instruction: u8) -> Option<fn(&mut Core) -> &mut Core> {
        let mut current_node = &self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            match current_node.children.get(&bit) {
                Some(node) => current_node = node,
                None => return None,
            }
        }

        current_node.function.clone()
    }
}

// This is a very messy prefix tree for looking up opcodes, but it works for now at least.
// All insert code was generated using the python helper script.
pub fn gen_trie() -> Trie {
    let mut trie: Trie = Trie::new();

    trie.insert(&0x69_u8, "ADC,IMM,2,2,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x65_u8, "ADC,ZP,2,3,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x75_u8, "ADC,ZPX,2,4,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x6d_u8, "ADC,ABS,3,4,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x7d_u8, "ADC,ABSX,3,4,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x79_u8, "ADC,ABSY,3,4,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x61_u8, "ADC,INDX,2,6,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x71_u8, "ADC,INDY,2,5,CZidbVN".to_string(), Some(adc));
    trie.insert(&0x29_u8, "AND,IMM,2,2,cZidbvN".to_string(), Some(and));
    trie.insert(&0x25_u8, "AND,ZP,2,3,cZidbvN".to_string(), Some(and));
    trie.insert(&0x35_u8, "AND,ZPX,2,4,cZidbvN".to_string(), Some(and));
    trie.insert(&0x2d_u8, "AND,ABS,3,4,cZidbvN".to_string(), Some(and));
    trie.insert(&0x3d_u8, "AND,ABSX,3,4,cZidbvN".to_string(), Some(and));
    trie.insert(&0x39_u8, "AND,ABSY,3,4,cZidbvN".to_string(), Some(and));
    trie.insert(&0x21_u8, "AND,INDX,2,6,cZidbvN".to_string(), Some(and));
    trie.insert(&0x31_u8, "AND,INDY,2,5,cZidbvN".to_string(), Some(and));
    trie.insert(&0x0a_u8, "ASL,ACC,1,2,CZidbvN".to_string(), Some(asl));
    trie.insert(&0x06_u8, "ASL,ZP,2,5,CZidbvN".to_string(), Some(asl));
    trie.insert(&0x16_u8, "ASL,ZPX,2,6,CZidbvN".to_string(), Some(asl));
    trie.insert(&0x0e_u8, "ASL,ABS,3,6,CZidbvN".to_string(), Some(asl));
    trie.insert(&0x1e_u8, "ASL,ABSX,3,7,CZidbvN".to_string(), Some(asl));
    trie.insert(&0x90_u8, "BCC,REL,2,2/3,czidbvn".to_string(), Some(bcc));
    trie.insert(&0xB0_u8, "BCS,REL,2,2/3,czidbvn".to_string(), Some(bcs));
    trie.insert(&0xF0_u8, "BEQ,REL,2,2/3,czidbvn".to_string(), Some(beq));
    trie.insert(&0x30_u8, "BMI,REL,2,2/3,czidbvn".to_string(), Some(bmi));
    trie.insert(&0xD0_u8, "BNE,REL,2,2/3,czidbvn".to_string(), Some(bne));
    trie.insert(&0x10_u8, "BPL,REL,2,2/3,czidbvn".to_string(), Some(bpl));
    trie.insert(&0x50_u8, "BVC,REL,2,2/3,czidbvn".to_string(), Some(bvc));
    trie.insert(&0x70_u8, "BVS,REL,2,2/3,czidbvn".to_string(), Some(bvs));
    trie.insert(&0x24_u8, "BIT,ZP,2,3,cZidbVN".to_string(), Some(bit));
    trie.insert(&0x2c_u8, "BIT,ABS,3,4,cZidbVN".to_string(), Some(bit));
    trie.insert(&0x00_u8, "BRK,IMP,1,7,czidbvn".to_string(), Some(brk));
    trie.insert(&0x18_u8, "CLC,IMP,1,2,Czidbvn".to_string(), Some(clc));
    trie.insert(&0xd8_u8, "CLD,IMP,1,2,cziDbvn".to_string(), Some(cld));
    trie.insert(&0x58_u8, "CLI,IMP,1,2,czIdbvn".to_string(), Some(cli));
    trie.insert(&0xb8_u8, "CLV,IMP,1,2,czidbVn".to_string(), Some(clv));
    trie.insert(&0xea_u8, "NOP,IMP,1,2,czidbvn".to_string(), Some(nop));
    trie.insert(&0x48_u8, "PHA,IMP,1,3,czidbvn".to_string(), Some(pha));
    trie.insert(&0x68_u8, "PLA,IMP,1,4,cZidbvN".to_string(), Some(pla));
    trie.insert(&0x08_u8, "PHP,IMP,1,3,czidbvn".to_string(), Some(php));
    trie.insert(&0x28_u8, "PLP,IMP,1,4,CZIDBVN".to_string(), Some(plp));
    trie.insert(&0x40_u8, "RTI,IMP,1,6,czidbvn".to_string(), Some(rti));
    trie.insert(&0x60_u8, "RTS,IMP,1,6,czidbvn".to_string(), Some(rts));
    trie.insert(&0x38_u8, "SEC,IMP,1,2,Czidbvn".to_string(), Some(sec));
    trie.insert(&0xf8_u8, "SED,IMP,1,2,cziDbvn".to_string(), Some(sed));
    trie.insert(&0x78_u8, "SEI,IMP,1,2,czIdbvn".to_string(), Some(sei));
    trie.insert(&0xaa_u8, "TAX,IMP,1,2,cZidbvN".to_string(), Some(tax));
    trie.insert(&0x8a_u8, "TXA,IMP,1,2,cZidbvN".to_string(), Some(txa));
    trie.insert(&0xa8_u8, "TAY,IMP,1,2,cZidbvN".to_string(), Some(tay));
    trie.insert(&0x98_u8, "TYA,IMP,1,2,cZidbvN".to_string(), Some(tya));
    trie.insert(&0xba_u8, "TSX,IMP,1,2,cZidbvN".to_string(), Some(tsx));
    trie.insert(&0x9a_u8, "TXS,IMP,1,2,czidbvn".to_string(), Some(txs));
    trie.insert(&0xc9_u8, "CMP,IMM,2,2,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xc5_u8, "CMP,ZP,2,3,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xd5_u8, "CMP,ZPX,2,4,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xcd_u8, "CMP,ABS,3,4,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xdd_u8, "CMP,ABSX,3,4,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xd9_u8, "CMP,ABSY,3,4,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xc1_u8, "CMP,INDX,2,6,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xd1_u8, "CMP,INDY,2,5,CZidbvN".to_string(), Some(cmp));
    trie.insert(&0xe0_u8, "CPX,IMM,2,2,CZidbvN".to_string(), Some(cpx));
    trie.insert(&0xe4_u8, "CPX,ZP,2,3,CZidbvN".to_string(), Some(cpx));
    trie.insert(&0xec_u8, "CPX,ABS,3,4,CZidbvN".to_string(), Some(cpx));
    trie.insert(&0xc0_u8, "CPY,IMM,2,2,CZidbvN".to_string(), Some(cpy));
    trie.insert(&0xc4_u8, "CPY,ZP,2,3,CZidbvN".to_string(), Some(cpy));
    trie.insert(&0xcc_u8, "CPY,ABS,3,4,CZidbvN".to_string(), Some(cpy));
    trie.insert(&0xc6_u8, "DEC,ZP,2,5,cZidbvN".to_string(), Some(dec));
    trie.insert(&0xd6_u8, "DEC,ZPX,2,6,cZidbvN".to_string(), Some(dec));
    trie.insert(&0xce_u8, "DEC,ABS,3,6,cZidbvN".to_string(), Some(dec));
    trie.insert(&0xde_u8, "DEC,ABSX,3,7,cZidbvN".to_string(), Some(dec));
    trie.insert(&0xca_u8, "DEX,IMP,1,2,cZidbvN".to_string(), Some(dex));
    trie.insert(&0x88_u8, "DEY,IMP,1,2,cZidbvN".to_string(), Some(dey));
    trie.insert(&0xe8_u8, "INX,IMP,1,2,cZidbvN".to_string(), Some(inx));
    trie.insert(&0xc8_u8, "INY,IMP,1,2,cZidbvN".to_string(), Some(iny));
    trie.insert(&0x49_u8, "EOR,IMM,2,2,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x45_u8, "EOR,ZP,2,3,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x55_u8, "EOR,ZPX,2,4,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x4d_u8, "EOR,ABS,3,4,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x5d_u8, "EOR,ABSX,3,4,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x59_u8, "EOR,ABSY,3,4,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x41_u8, "EOR,INDX,2,6,cZidbvN".to_string(), Some(eor));
    trie.insert(&0x51_u8, "EOR,INDY,2,5,cZidbvN".to_string(), Some(eor));
    trie.insert(&0xe6_u8, "INC,ZP,2,5,cZidbvN".to_string(), Some(inc));
    trie.insert(&0xf6_u8, "INC,ZPX,2,6,cZidbvN".to_string(), Some(inc));
    trie.insert(&0xee_u8, "INC,ABS,3,6,cZidbvN".to_string(), Some(inc));
    trie.insert(&0xfe_u8, "INC,ABSX,3,7,cZidbvN".to_string(), Some(inc));
    trie.insert(&0x4c_u8, "JMP,ABS,3,3,czidbvn".to_string(), Some(jmp));
    trie.insert(&0x6c_u8, "JMP,IND,3,5,czidbvn".to_string(), Some(jmp));
    trie.insert(&0x20_u8, "JSR,ABS,3,6,czidbvn".to_string(), Some(jsr));
    trie.insert(&0xa9_u8, "LDA,IMM,2,2,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xa5_u8, "LDA,ZP,2,3,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xb5_u8, "LDA,ZPX,2,4,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xad_u8, "LDA,ABS,3,4,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xbd_u8, "LDA,ABSX,3,4,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xb9_u8, "LDA,ABSY,3,4,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xa1_u8, "LDA,INDX,2,6,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xb1_u8, "LDA,INDY,2,5,cZidbvN".to_string(), Some(lda));
    trie.insert(&0xa2_u8, "LDX,IMM,2,2,cZidbvN".to_string(), Some(ldx));
    trie.insert(&0xa6_u8, "LDX,ZP,2,3,cZidbvN".to_string(), Some(ldx));
    trie.insert(&0xb6_u8, "LDX,ZPY,2,4,cZidbvN".to_string(), Some(ldx));
    trie.insert(&0xae_u8, "LDX,ABS,3,4,cZidbvN".to_string(), Some(ldx));
    trie.insert(&0xbe_u8, "LDX,ABSY,3,4,cZidbvN".to_string(), Some(ldx));
    trie.insert(&0xa0_u8, "LDY,IMM,2,2,cZidbvN".to_string(), Some(ldy));
    trie.insert(&0xa4_u8, "LDY,ZP,2,3,cZidbvN".to_string(), Some(ldy));
    trie.insert(&0xb4_u8, "LDY,ZPX,2,4,cZidbvN".to_string(), Some(ldy));
    trie.insert(&0xac_u8, "LDY,ABS,3,4,cZidbvN".to_string(), Some(ldy));
    trie.insert(&0xbc_u8, "LDY,ABSX,3,4,cZidbvN".to_string(), Some(ldy));
    trie.insert(&0x4a_u8, "LSR,ACC,1,2,CZidbvN".to_string(), Some(lsr));
    trie.insert(&0x46_u8, "LSR,ZP,2,5,CZidbvN".to_string(), Some(lsr));
    trie.insert(&0x56_u8, "LSR,ZPX,2,6,CZidbvN".to_string(), Some(lsr));
    trie.insert(&0x4e_u8, "LSR,ABS,3,6,CZidbvN".to_string(), Some(lsr));
    trie.insert(&0x5e_u8, "LSR,ABSX,3,7,CZidbvN".to_string(), Some(lsr));
    trie.insert(&0x09_u8, "ORA,IMM,2,2,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x05_u8, "ORA,ZP,2,3,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x15_u8, "ORA,ZPX,2,4,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x0d_u8, "ORA,ABS,3,4,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x1d_u8, "ORA,ABSX,3,4,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x19_u8, "ORA,ABSY,3,4,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x01_u8, "ORA,INDX,2,6,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x11_u8, "ORA,INDY,2,5,cZidbvN".to_string(), Some(ora));
    trie.insert(&0x2a_u8, "ROL,ACC,1,2,CZidbvN".to_string(), Some(rol));
    trie.insert(&0x26_u8, "ROL,ZP,2,5,CZidbvN".to_string(), Some(rol));
    trie.insert(&0x36_u8, "ROL,ZPX,2,6,CZidbvN".to_string(), Some(rol));
    trie.insert(&0x2e_u8, "ROL,ABS,3,6,CZidbvN".to_string(), Some(rol));
    trie.insert(&0x3e_u8, "ROL,ABSX,3,7,CZidbvN".to_string(), Some(rol));
    trie.insert(&0x6a_u8, "ROR,ACC,1,2,CZidbvN".to_string(), Some(ror));
    trie.insert(&0x66_u8, "ROR,ZP,2,5,CZidbvN".to_string(), Some(ror));
    trie.insert(&0x76_u8, "ROR,ZPX,2,6,CZidbvN".to_string(), Some(ror));
    trie.insert(&0x7e_u8, "ROR,ABS,3,6,CZidbvN".to_string(), Some(ror));
    trie.insert(&0x6e_u8, "ROR,ABSX,3,7,CZidbvN".to_string(), Some(ror));
    trie.insert(&0xe9_u8, "SBC,IMM,2,2,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xe5_u8, "SBC,ZP,2,3,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xf5_u8, "SBC,ZPX,2,4,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xed_u8, "SBC,ABS,3,4,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xfd_u8, "SBC,ABSX,3,4,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xf9_u8, "SBC,ABSY,3,4,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xe1_u8, "SBC,INDX,2,6,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0xf1_u8, "SBC,INDY,2,5,CZidbVN".to_string(), Some(sbc));
    trie.insert(&0x85_u8, "STA,ZP,2,3,czidbvn".to_string(), Some(sta));
    trie.insert(&0x95_u8, "STA,ZPX,2,4,czidbvn".to_string(), Some(sta));
    trie.insert(&0x8d_u8, "STA,ABS,3,4,czidbvn".to_string(), Some(sta));
    trie.insert(&0x9d_u8, "STA,ABSX,3,5,czidbvn".to_string(), Some(sta));
    trie.insert(&0x99_u8, "STA,ABSY,3,5,czidbvn".to_string(), Some(sta));
    trie.insert(&0x81_u8, "STA,INDX,2,6,czidbvn".to_string(), Some(sta));
    trie.insert(&0x91_u8, "STA,INDY,2,6,czidbvn".to_string(), Some(sta));
    trie.insert(&0x86_u8, "STX,ZP,2,3,czidbvn".to_string(), Some(stx));
    trie.insert(&0x96_u8, "STX,ZPY,2,4,czidbvn".to_string(), Some(stx));
    trie.insert(&0x8e_u8, "STX,ABS,3,4,czidbvn".to_string(), Some(stx));
    trie.insert(&0x84_u8, "STY,ZP,2,3,czidbvn".to_string(), Some(sty));
    trie.insert(&0x94_u8, "STY,ZPX,2,4,czidbvn".to_string(), Some(sty));
    trie.insert(&0x8c_u8, "STY,ABS,3,4,czidbvn".to_string(), Some(sty));
                                                
    trie
}