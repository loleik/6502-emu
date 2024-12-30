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
pub struct DisNode {
    is_prefix: bool,
    children: HashMap<u8, DisNode>,
    instruction: Option<String>,
}

// Trie struct
#[derive(Default, Debug)]
pub struct DisTrie {
    root: DisNode,
}

impl DisTrie {
    // Create a new trie.
    pub fn new() -> Self {
        DisTrie { root: DisNode::default() }
    }

    // Inserting a new entry into a trie.
    pub fn insert(&mut self, instruction: &u8, info: String) {
        let mut current_node = &mut self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            current_node = current_node.children.entry(bit).or_default();
        }

        current_node.is_prefix = true;
        current_node.instruction = Some(info);
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
    pub fn get_instruction(&self, instruction: u8) -> Option<&String> {
        let mut current_node = &self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            match current_node.children.get(&bit) {
                Some(node) => current_node = node,
                None => return None,
            }
        }

        current_node.instruction.as_ref()
    }
}