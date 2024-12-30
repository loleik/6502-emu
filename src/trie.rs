use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct DisNode {
    is_prefix: bool,
    children: HashMap<u8, DisNode>,
    instruction: Option<String>,
}

#[derive(Default, Debug)]
pub struct DisTrie {
    root: DisNode,
}

impl DisTrie {
    pub fn new() -> Self {
        DisTrie { root: DisNode::default() }
    }

    pub fn insert(&mut self, instruction: &u8, info: String) {
        let mut current_node = &mut self.root;

        for i in 0..8 {
            let bit = (instruction >> i) & 1;

            current_node = current_node.children.entry(bit).or_default();
        }

        current_node.is_prefix = true;
        current_node.instruction = Some(info);
    }

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