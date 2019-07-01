#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod cpu;
mod io;
mod memory;
mod sprite;

use cpu::{load_next_instruction, Cpu};
use io::{Display, Point, Window};
use memory::Memory;

use processing;
use processing::Screen;

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

const NANOS_PER_MILLI: u32 = 1000000;
const MILLIS_PER_INSTRUCTION: u32 = 16;

pub fn start_emulator(filename: String) {
    let mut ram = Memory::new();

    {
        let file = fs::read(&filename).unwrap();
        ram.write(cpu::PROGRAM_START, &file[..]);
    }

    let mut window = Window::new();
    let mut display = Display::new();
    let mut cpu = Cpu::new(false);

    loop {
        // TODO(Joshua): Make this two loops, outer going at 60 hz, inner
        // performing as many instructions as are required to bring the executed
        // instructions up to the instructions/second count passed in

        let instr = load_next_instruction(&mut cpu, &ram);
        cpu.execute_instruction(instr, &mut ram, &mut display, &mut window);

        let events = window.draw_display(&display).unwrap();
        window.process_events(events);

        std::thread::sleep(std::time::Duration::new(
            0,
            NANOS_PER_MILLI * MILLIS_PER_INSTRUCTION,
        ));
    }
}
