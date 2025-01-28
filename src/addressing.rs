use crate::system::Core;

/* Probably not needed just here in case
pub fn immediate(core: Core) -> u8 {
}
pub fn accumulator(core: Core) -> u8 {
}*/

pub fn zero_page(core: &mut Core) -> u8 {
    core.memory[core.pc as usize + 1]
}

pub fn zero_page_x(core: &mut Core) -> u8 {
    core.memory[(core.pc as usize + 1) + core.ix as usize]
}

pub fn zero_page_y(core: &mut Core) -> u8 {
    core.memory[(core.pc as usize + 1) + core.iy as usize]
}

pub fn absolute(core: &mut Core) -> u16 {
    let pcl: u8 = core.memory[core.pc as usize + 1];
    let pch: u8 = core.memory[core.pc as usize + 2];

    ((pch as u16) << 8) | (pcl as u16)
}