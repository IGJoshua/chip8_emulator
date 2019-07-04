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
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const NANOS_PER_MILLI: u64 = 1000000;
const NANOS_PER_SECOND: u64 = NANOS_PER_MILLI * 1000;

pub fn start_emulator(filename: String, cycles_per_second: u64) {
    let mut ram = Memory::new();

    {
        let file = fs::read(&filename).unwrap();
        ram.write(cpu::PROGRAM_START, &file[..]);
    }

    let mut window = Window::new();
    let mut display = Display::new();
    let mut cpu = Cpu::new(false);

    let iteration_duration = Duration::from_nanos(NANOS_PER_SECOND / 60);
    let cycles_per_iteration = cycles_per_second / 60;

    println!("Desired millis per frame: {}", iteration_duration.as_millis());

    let mut frame_start = SystemTime::now();
    loop {
        println!("Millis for last frame: {}", frame_start.elapsed().unwrap().as_millis());
        frame_start = SystemTime::now();
        //let frame_start_time = SystemTime::now();

        for _ in 0..cycles_per_iteration + 1 {
            let instr = load_next_instruction(&mut cpu, &ram);
            cpu.execute_instruction(instr, &mut ram, &mut display, &mut window);
        }

        cpu.step_clocks();

        println!("Millis for instruction execution: {}", frame_start.elapsed().unwrap().as_millis());

        let events = window.draw_display(&display).unwrap();
        println!("Millis for window drawing: {}", frame_start.elapsed().unwrap().as_millis());

        window.process_events(events);

        println!("Millis for events: {}", frame_start.elapsed().unwrap().as_millis());

        let frame_duration = frame_start.elapsed().unwrap();
        if frame_duration < iteration_duration {
            sleep(iteration_duration - frame_duration);
        }
    }
}
