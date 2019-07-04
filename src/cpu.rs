use crate::io;
use crate::io::{Display, Window};
use crate::memory;
use crate::memory::Memory;
use crate::sprite;
use rand;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::u8;

const STACK_TOP: usize = 0x200;
pub const PROGRAM_START: usize = 0x200;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vx(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr(u16);

pub struct Cpu {
    registers: Registers,
    rng: rand::rngs::ThreadRng,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Cls,
    Ret,
    Sys(Addr),
    Jmp(Addr),
    Call(Addr),
    SkipEq(Vx, u8),
    SkipNotEq(Vx, u8),
    SkipEqVx(Vx, Vx),
    Load(Vx, u8),
    Add(Vx, u8),
    LoadVx(Vx, Vx),
    Or(Vx, Vx),
    And(Vx, Vx),
    XOr(Vx, Vx),
    AddVx(Vx, Vx),
    SubVx(Vx, Vx),
    ShiftRight(Vx),
    SubN(Vx, Vx),
    ShiftLeft(Vx),
    SkipNotEqVx(Vx, Vx),
    LoadI(Addr),
    JmpV0(Addr),
    Rand(Vx, u8),
    Draw(Vx, Vx, u8),
    SkipKeyPressed(Vx),
    SkipKeyNotPressed(Vx),
    LoadDelay(Vx),
    LoadKey(Vx),
    SetDelay(Vx),
    SetSound(Vx),
    AddI(Vx),
    LoadFont(Vx),
    LoadBcd(Vx),
    StoreRegisters(Vx),
    LoadRegisters(Vx),
}

impl Addr {
    fn new(nibble1: u8, nibble2: u8, nibble3: u8) -> Addr {
        Addr(((nibble1 as u16) << 8) | ((nibble2 as u16) << 4) | nibble3 as u16)
    }
}

impl Cpu {
    pub fn new(eti_mode: bool) -> Cpu {
        Cpu {
            registers: Registers {
                vx: [0; NUM_REGISTERS],
                i: 0,
                delay: 0,
                sound: 0,
                pc: if eti_mode {
                    ETI_600_START as u16
                } else {
                    PROGRAM_START as u16
                },
                sp: 0,
            },
            rng: rand::thread_rng(),
        }
    }

    pub fn step_clocks(&mut self) {
        if self.registers.delay > 0 {
            self.registers.delay -= 1;
        }
        if self.registers.sound > 0 {
            self.registers.sound -= 1;
        }
    }

    pub fn execute_instruction(
        &mut self,
        instr: Instruction,
        ram: &mut Memory,
        display: &mut Display,
        window: &mut Window,
    ) {
        match instr {
            Instruction::Cls => {
                display.clear_screen();
            }
            Instruction::Ret => {
                self.registers.pc = self.stack_pop(ram);
            }
            Instruction::Sys(_) => {}
            Instruction::Jmp(addr) => {
                self.registers.pc = addr.0;
            }
            Instruction::Call(addr) => {
                self.stack_push(ram, self.registers.pc);
                self.registers.pc = addr.0;
            }
            Instruction::SkipEq(idx, byte) => {
                if self.registers.vx[idx.0 as usize] == byte {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipNotEq(idx, byte) => {
                if self.registers.vx[idx.0 as usize] != byte {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipEqVx(x, y) => {
                if self.registers.vx[x.0 as usize] == self.registers.vx[y.0 as usize] {
                    self.registers.pc += 2;
                }
            }
            Instruction::Load(idx, byte) => {
                self.registers.vx[idx.0 as usize] = byte;
            }
            Instruction::Add(idx, byte) => {
                self.registers.vx[idx.0 as usize] =
                    self.registers.vx[idx.0 as usize].wrapping_add(byte);
            }
            Instruction::LoadVx(x, y) => {
                self.registers.vx[x.0 as usize] = self.registers.vx[y.0 as usize];
            }
            Instruction::Or(x, y) => {
                self.registers.vx[x.0 as usize] |= self.registers.vx[y.0 as usize];
            }
            Instruction::And(x, y) => {
                self.registers.vx[x.0 as usize] &= self.registers.vx[y.0 as usize];
            }
            Instruction::XOr(x, y) => {
                self.registers.vx[x.0 as usize] ^= self.registers.vx[y.0 as usize];
            }
            Instruction::AddVx(x, y) => {
                let result =
                    self.registers.vx[x.0 as usize] as u16 + self.registers.vx[y.0 as usize] as u16;
                let overflow = result > 0xFF;

                self.registers.vx[x.0 as usize] = (result & 0xFF) as u8;
                self.registers.vx[0xF] = if overflow { 1 } else { 0 };
            }
            Instruction::SubVx(x, y) => {
                self.registers.vx[0xF] =
                    if self.registers.vx[x.0 as usize] > self.registers.vx[y.0 as usize] {
                        1
                    } else {
                        0
                    };
                self.registers.vx[x.0 as usize] =
                    self.registers.vx[x.0 as usize].wrapping_sub(self.registers.vx[y.0 as usize]);
            }
            Instruction::ShiftRight(idx) => {
                self.registers.vx[0xF] = self.registers.vx[idx.0 as usize] & 0x1;

                self.registers.vx[idx.0 as usize] >>= 1;
            }
            Instruction::SubN(x, y) => {
                self.registers.vx[0xF] =
                    if self.registers.vx[y.0 as usize] > self.registers.vx[x.0 as usize] {
                        1
                    } else {
                        0
                    };

                self.registers.vx[x.0 as usize] =
                    self.registers.vx[y.0 as usize].wrapping_sub(self.registers.vx[x.0 as usize]);
            }
            Instruction::ShiftLeft(idx) => {
                self.registers.vx[0xF] = self.registers.vx[idx.0 as usize] >> 7 & 0x1;

                self.registers.vx[idx.0 as usize] <<= 1;
            }
            Instruction::SkipNotEqVx(x, y) => {
                if self.registers.vx[x.0 as usize] != self.registers.vx[y.0 as usize] {
                    self.registers.pc += 2;
                }
            }
            Instruction::LoadI(addr) => {
                self.registers.i = addr.0 & 0x0FFF;
            }
            Instruction::JmpV0(addr) => {
                self.registers.pc = self.registers.vx[0] as u16 + addr.0;
            }
            Instruction::Rand(idx, mask) => {
                self.registers.vx[idx.0 as usize] = (self.rng.gen_range(0, 256) as u8) & mask;
            }
            Instruction::Draw(x, y, bytes) => {
                let x = self.registers.vx[x.0 as usize];
                let y = self.registers.vx[y.0 as usize];
                let mut rows = vec![0; bytes as usize];
                ram.read(self.registers.i as usize, &mut rows[..]);
                let sprite = sprite::Sprite { rows };
                let collision = display.draw_sprite(io::Point(x, y), sprite);
                self.registers.vx[0xF as usize] = if collision { 1 } else { 0 };
            }
            Instruction::SkipKeyPressed(idx) => {
                if window.key_down(idx.0) {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipKeyNotPressed(idx) => {
                if !window.key_down(idx.0) {
                    self.registers.pc += 2;
                }
            }
            Instruction::LoadDelay(idx) => {
                self.registers.vx[idx.0 as usize] = self.registers.delay;
            }
            Instruction::LoadKey(idx) => {
                self.registers.vx[idx.0 as usize] = window.wait_for_key();
            }
            Instruction::SetDelay(idx) => {
                self.registers.delay = self.registers.vx[idx.0 as usize];
            }
            Instruction::SetSound(idx) => {
                self.registers.sound = self.registers.vx[idx.0 as usize];
            }
            Instruction::AddI(idx) => {
                self.registers.vx[0xF] =
                    if self.registers.i as u32 + self.registers.vx[idx.0 as usize] as u32 > 0xFFF {
                        1
                    } else {
                        0
                    };
                self.registers.i = self
                    .registers
                    .i
                    .wrapping_add(self.registers.vx[idx.0 as usize] as u16);
            }
            Instruction::LoadFont(idx) => {
                self.registers.i =
                    sprite::SPRITE_SIZE as u16 * self.registers.vx[idx.0 as usize] as u16;
            }
            Instruction::LoadBcd(idx) => {
                let mut bcd = [0; 3];
                let num = self.registers.vx[idx.0 as usize];
                bcd[0] = num / 100;
                bcd[1] = (num % 100) / 10;
                bcd[2] = num % 10;
                ram.write(self.registers.i as usize, &bcd[..]);
            }
            Instruction::StoreRegisters(idx) => {
                let mut buf = vec![0; (idx.0 + 1) as usize];
                for i in 0..(idx.0 + 1) as usize {
                    buf[i] = self.registers.vx[i];
                }
                ram.write(self.registers.i as usize, &buf[..]);
            }
            Instruction::LoadRegisters(idx) => {
                let mut buf = vec![0; (idx.0 + 1) as usize];
                ram.read(self.registers.i as usize, &mut buf[..]);
                for i in 0..(idx.0 + 1) as usize {
                    self.registers.vx[i] = buf[i];
                }
            }
        }
    }

    fn stack_pop(&mut self, ram: &Memory) -> u16 {
        let mut addr: [u8; 2] = [0; 2];
        ram.read(STACK_TOP - (self.registers.sp as usize * 2), &mut addr[..]);
        self.registers.sp -= 1;
        construct_short(addr[0], addr[1])
    }

    fn stack_push(&mut self, ram: &mut Memory, addr: u16) {
        let addr: [u8; 2] = [((addr >> 8) & 0xFF) as u8, (addr & 0xFF) as u8];
        self.registers.sp += 1;
        ram.write(STACK_TOP - (self.registers.sp as usize * 2), &addr[..]);
    }
}

pub fn load_next_instruction(cpu: &mut Cpu, ram: &Memory) -> Instruction {
    let mut instr: [u8; 2] = [0; 2];
    ram.read(cpu.registers.pc as usize, &mut instr);
    cpu.registers.pc += 2;

    // TODO(Joshua): Proper error handling, or just surfacing the option
    read_instruction(instr[0], instr[1]).unwrap()
}

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
        (0x3, x, hk, lk) => Some(Instruction::SkipEq(Vx(x), construct_byte(hk, lk))),
        (0x4, x, hk, lk) => Some(Instruction::SkipNotEq(Vx(x), construct_byte(hk, lk))),
        (0x5, x, y, 0x0) => Some(Instruction::SkipEqVx(Vx(x), Vx(y))),
        (0x6, x, hk, lk) => Some(Instruction::Load(Vx(x), construct_byte(hk, lk))),
        (0x7, x, hk, lk) => Some(Instruction::Add(Vx(x), construct_byte(hk, lk))),
        (0x8, x, y, 0x0) => Some(Instruction::AddVx(Vx(x), Vx(y))),
        (0x8, x, y, 0x1) => Some(Instruction::Or(Vx(x), Vx(y))),
        (0x8, x, y, 0x2) => Some(Instruction::And(Vx(x), Vx(y))),
        (0x8, x, y, 0x3) => Some(Instruction::XOr(Vx(x), Vx(y))),
        (0x8, x, y, 0x4) => Some(Instruction::AddVx(Vx(x), Vx(y))),
        (0x8, x, y, 0x5) => Some(Instruction::SubVx(Vx(x), Vx(y))),
        (0x8, x, _, 0x6) => Some(Instruction::ShiftRight(Vx(x))),
        (0x8, x, y, 0x7) => Some(Instruction::SubN(Vx(x), Vx(y))),
        (0x8, x, _, 0xE) => Some(Instruction::ShiftLeft(Vx(x))),
        (0x9, x, y, 0x0) => Some(Instruction::SkipNotEqVx(Vx(x), Vx(y))),
        (0xA, a, b, c) => Some(Instruction::LoadI(Addr::new(a, b, c))),
        (0xB, a, b, c) => Some(Instruction::JmpV0(Addr::new(a, b, c))),
        (0xC, x, hk, lk) => Some(Instruction::Rand(Vx(x), construct_byte(hk, lk))),
        (0xD, x, y, k) => Some(Instruction::Draw(Vx(x), Vx(y), k)),
        (0xE, x, 0x9, 0xE) => Some(Instruction::SkipKeyPressed(Vx(x))),
        (0xE, x, 0xA, 0x1) => Some(Instruction::SkipKeyNotPressed(Vx(x))),
        (0xF, x, 0x0, 0x7) => Some(Instruction::LoadDelay(Vx(x))),
        (0xF, x, 0x0, 0xA) => Some(Instruction::LoadKey(Vx(x))),
        (0xF, x, 0x1, 0x5) => Some(Instruction::SetDelay(Vx(x))),
        (0xF, x, 0x1, 0x8) => Some(Instruction::SetSound(Vx(x))),
        (0xF, x, 0x1, 0xE) => Some(Instruction::AddI(Vx(x))),
        (0xF, x, 0x2, 0x9) => Some(Instruction::LoadFont(Vx(x))),
        (0xF, x, 0x3, 0x3) => Some(Instruction::LoadBcd(Vx(x))),
        (0xF, x, 0x5, 0x5) => Some(Instruction::StoreRegisters(Vx(x))),
        (0xF, x, 0x6, 0x5) => Some(Instruction::LoadRegisters(Vx(x))),
        _ => None,
    }
}

fn construct_byte(high_nibble: u8, low_nibble: u8) -> u8 {
    ((high_nibble & 0x0F) << 4) | (low_nibble & 0x0F)
}

fn construct_short(high_bytes: u8, low_bytes: u8) -> u16 {
    ((high_bytes as u16) << 8) | (low_bytes as u16)
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
        assert_eq!(
            Instruction::Load(Vx(2), 7),
            read_instruction(0x62, 0x7).unwrap()
        );
    }
}
