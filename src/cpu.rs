use crate::memory;
use crate::memory::Memory;
use crate::io;
use crate::io::Display;
use std::rc::Rc;
use std::cell::RefCell;

const PROGRAM_START: usize = 0x200;
const ETI_600_START: usize = 0x600;
const NUM_REGISTERS: usize = 0x10;

struct Registers {
    vx: [u8; NUM_REGISTERS],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
}

pub struct Cpu {
    ram: Memory,
    display: Rc<RefCell<Display>>,
    registers: Registers,
}

impl Cpu {
    pub fn new(ram: Memory, display: Rc<RefCell<Display>>) -> Cpu {
        Cpu {
            ram,
            display,
            registers: Registers {
                vx: [0; NUM_REGISTERS],
                i: 0,
                delay: 0,
                sound: 0,
                pc: 0,
                sp: 0,
            }
        }
    }
}
