#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod cpu;
mod io;
mod memory;
mod sprite;

use cpu::Cpu;
use io::{Display, Point};
use memory::Memory;

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn start_emulator(filename: String) {
    let mut ram = Memory::new();
    let file = fs::read(&filename).unwrap();
    ram.write(cpu::PROGRAM_START, &file[..]);

    let display = Rc::new(RefCell::new(Display::new()));
    let mut cpu = Cpu::new(ram, Rc::clone(&display), false);

    loop {
        let instr = cpu.load_next_instruction();
        cpu.execute_instruction(instr);

        display.borrow_mut().flip();

        std::thread::sleep(std::time::Duration::new(0, 16000000));
    }
}
