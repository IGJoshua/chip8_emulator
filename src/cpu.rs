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
    Cls,
    Ret,
    Sys(Addr),
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

impl Addr {
    fn new(nibble1: u8, nibble2: u8, nibble3: u8) -> Addr {
        Addr(((nibble1 as u16) << 8) | ((nibble2 as u16) << 4) | nibble3 as u16)
    }
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

impl Instruction {
    fn read_instruction(high_byte: u8, low_byte: u8) -> Option<Instruction> {
        let nibble1 = (high_byte >> 4) & 0x0F;
        let nibble2 = high_byte & 0x0F;
        let nibble3 = (low_byte >> 4) & 0x0F;
        let nibble4 = low_byte & 0x0F;

        match (nibble1, nibble2, nibble3, nibble4) {
            (0x0, 0x0, 0xE, 0x0) => Some(Instruction::Cls),
            (0x0, 0x0, 0xE, 0xE) => Some(Instruction::Ret),
            (0x0, a, b, c) => Some(Instruction::Sys(Addr::new(a, b, c))),
            (0x1, a, b, c) => Some(Instruction::Jmp(Addr::new(a, b, c))),
            (0x2, a, b, c) => Some(Instruction::Call(Addr::new(a, b, c))),
            (0x3, x, hk, lk) => Some(Instruction::SkipEq(VxIdx(x), construct_byte(hk, lk))),
            (0x4, x, hk, lk) => Some(Instruction::SkipNotEq(VxIdx(x), construct_byte(hk, lk))),
            (0x5, x, y, 0x0) => Some(Instruction::SkipEqVx(VxIdx(x), VxIdx(y))),
            (0x6, x, hk, lk) => Some(Instruction::Load(VxIdx(x), construct_byte(hk, lk))),
            (0x7, x, hk, lk) => Some(Instruction::Add(VxIdx(x), construct_byte(hk, lk))),
            (0x8, x, y, 0x0) => Some(Instruction::AddVx(VxIdx(x), VxIdx(y))),
            (0x8, x, y, 0x1) => Some(Instruction::Or(VxIdx(x), VxIdx(y))),
            (0x8, x, y, 0x2) => Some(Instruction::And(VxIdx(x), VxIdx(y))),
            (0x8, x, y, 0x3) => Some(Instruction::XOr(VxIdx(x), VxIdx(y))),
            (0x8, x, y, 0x4) => Some(Instruction::AddVx(VxIdx(x), VxIdx(y))),
            (0x8, x, y, 0x5) => Some(Instruction::SubVx(VxIdx(x), VxIdx(y))),
            (0x8, x, _, 0x6) => Some(Instruction::ShiftRight(VxIdx(x))),
            (0x8, x, y, 0x7) => Some(Instruction::SubN(VxIdx(x), VxIdx(y))),
            (0x8, x, _, 0xE) => Some(Instruction::ShiftLeft(VxIdx(x))),
            (0x9, x, y, 0x0) => Some(Instruction::SkipNextEq(VxIdx(x), VxIdx(y))),
            (0xA, a, b, c) => Some(Instruction::LoadI(Addr::new(a, b, c))),
            (0xB, a, b, c) => Some(Instruction::JmpV0(Addr::new(a, b, c))),
            (0xC, x, hk, lk) => Some(Instruction::Rand(VxIdx(x), construct_byte(hk, lk))),
            (0xD, x, y, k) => Some(Instruction::Draw(VxIdx(x), VxIdx(y), k)),
            (0xE, x, 0x9, 0xE) => Some(Instruction::SkipKeyPressed(VxIdx(x))),
            (0xE, x, 0xA, 0x1) => Some(Instruction::SkipKeyNotPressed(VxIdx(x))),
            (0xF, x, 0x0, 0x7) => Some(Instruction::LoadDelay(VxIdx(x))),
            (0xF, x, 0x0, 0xA) => Some(Instruction::LoadKey(VxIdx(x))),
            (0xF, x, 0x1, 0x5) => Some(Instruction::SetDelay(VxIdx(x))),
            (0xF, x, 0x1, 0x8) => Some(Instruction::SetSound(VxIdx(x))),
            (0xF, x, 0x1, 0xE) => Some(Instruction::AddI(VxIdx(x))),
            (0xF, x, 0x2, 0x9) => Some(Instruction::LoadFont(VxIdx(x))),
            (0xF, x, 0x3, 0x3) => Some(Instruction::LoadBcd(VxIdx(x))),
            (0xF, x, 0x5, 0x5) => Some(Instruction::StoreRegisters(VxIdx(x))),
            (0xF, x, 0x6, 0x5) => Some(Instruction::LoadRegisters(VxIdx(x))),
            _ => None,
        }
    }
}

fn construct_byte(high_nibble: u8, low_nibble: u8) -> u8 {
    ((high_nibble & 0x0F) << 4) | (low_nibble & 0x0F)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construct_byte() {
        assert_eq!(0x12, construct_byte(0x1, 0x2));
    }

    #[test]
    fn test_read_instruction() {
        assert_eq!(Instruction::Load(VxIdx(2), 7), Instruction::read_instruction(0x62, 0x7).unwrap());
    }
}
