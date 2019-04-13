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
    vx: [Vx; NUM_REGISTERS],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vx(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VxIdx(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Addr(u16);

pub struct Cpu {
    ram: Memory,
    display: Rc<RefCell<Display>>,
    registers: Registers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Sys(Addr),
    Cls,
    Ret,
    Jmp(Addr),
    Call(Addr),
    SkipEq(VxIdx, u8),
    SkipNotEq(VxIdx, u8),
    SkipEqVx(VxIdx, VxIdx),
    Load(VxIdx, u8),
    Add(VxIdx, u8),
    LoadVx(VxIdx, VxIdx),
    Or(VxIdx, VxIdx),
    And(VxIdx, VxIdx),
    XOr(VxIdx, VxIdx),
    AddVx(VxIdx, VxIdx),
    SubVx(VxIdx, VxIdx),
    ShiftRight(VxIdx),
    SubN(VxIdx, VxIdx),
    ShiftLeft(VxIdx),
    SkipNextEq(VxIdx, VxIdx),
    LoadI(Addr),
    JmpV0(Addr),
    Rand(VxIdx, u8),
    Draw(VxIdx, VxIdx, u8),
    SkipKeyPressed(VxIdx),
    SkipKeyNotPressed(VxIdx),
    LoadDelay(VxIdx),
    LoadKey(VxIdx),
    SetDelay(VxIdx),
    SetSound(VxIdx),
    AddI(VxIdx),
    LoadFont(VxIdx),
    LoadBcd(VxIdx),
    StoreRegisters(VxIdx),
    LoadRegisters(VxIdx),
}

impl Cpu {
    pub fn new(ram: Memory, display: Rc<RefCell<Display>>) -> Cpu {
        Cpu {
            ram,
            display,
            registers: Registers {
                vx: [Vx(0); NUM_REGISTERS],
                i: 0,
                delay: 0,
                sound: 0,
                pc: 0,
                sp: 0,
            }
        }
    }
}
