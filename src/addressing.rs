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

pub fn absolute_x(core: &mut Core) -> u16 {
    let pcl: u8 = core.memory[core.pc as usize + 1];
    let pch: u8 = core.memory[core.pc as usize + 2];

    ((pch as u16) << 8) | (pcl as u16) + core.ix as u16
}

pub fn absolute_y(core: &mut Core) -> u16 {
    let pcl: u8 = core.memory[core.pc as usize + 1];
    let pch: u8 = core.memory[core.pc as usize + 2];

    ((pch as u16) << 8) | (pcl as u16) + core.iy as u16
}

pub fn indirect(core: &mut Core) -> u16 {
    // Get the low and high bytes of the pointer and combine them
    let ptl: u8 = core.memory[core.pc as usize + 1];
    let pth: u8 = core.memory[core.pc as usize + 2];

    let pointer: u16 = ((pth as u16) << 8) | (ptl as u16);

    // Get the low and high bytes of the actual target and combine them
    let tl: u8 = core.memory[pointer as usize];
    let th: u8 = core.memory[(pointer + 1) as usize];

    ((th as u16) << 8) | (tl as u16)
}

pub fn relative(core: &mut Core) -> i8 {
    core.memory[(core.pc as usize) + 1] as i8
}