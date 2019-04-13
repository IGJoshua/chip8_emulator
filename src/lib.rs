#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod cpu;
mod io;
mod memory;

use cpu::Cpu;
use io::Display;
use memory::Memory;

use std::rc::Rc;
use std::cell::RefCell;

pub fn start_emulator() {
    let ram = Memory::new();
    let display = Display::new();
    let cpu = Cpu::new(ram, Rc::new(RefCell::new(display)));
}
