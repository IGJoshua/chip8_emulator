#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod cpu;
mod io;
mod memory;

use cpu::Cpu;
use io::{Display, Point};
use memory::Memory;

use std::rc::Rc;
use std::cell::RefCell;

pub fn start_emulator() {
    let ram = Memory::new();
    let display = Rc::new(RefCell::new(Display::new()));
    let cpu = Cpu::new(ram, Rc::clone(&display));
}
