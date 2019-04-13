use std::fmt;

struct Sprite {
    rows: [u8; 5],
}

#[derive(Clone, Copy)]
pub struct Point(pub u8, pub u8);

const SPRITES: [Sprite; 0x10] = [Sprite { rows: [0xF0, 0x90, 0x90, 0x90, 0xF0], },
                                 Sprite { rows: [0x20, 0x60, 0x20, 0x20, 0x70], },
                                 Sprite { rows: [0xF0, 0x10, 0xF0, 0x80, 0xF0], },
                                 Sprite { rows: [0xF0, 0x10, 0xF0, 0x10, 0xF0], },
                                 Sprite { rows: [0x90, 0x90, 0xF0, 0x10, 0x10], },
                                 Sprite { rows: [0xF0, 0x80, 0xF0, 0x10, 0xF0], },
                                 Sprite { rows: [0xF0, 0x80, 0xF0, 0x90, 0xF0], },
                                 Sprite { rows: [0xF0, 0x10, 0x20, 0x40, 0x40], },
                                 Sprite { rows: [0xF0, 0x90, 0xF0, 0x90, 0xF0], },
                                 Sprite { rows: [0xF0, 0x90, 0xF0, 0x10, 0xF0], },
                                 Sprite { rows: [0xF0, 0x90, 0xF0, 0x90, 0x90], },
                                 Sprite { rows: [0xE0, 0x90, 0xE0, 0x90, 0xE0], },
                                 Sprite { rows: [0xF0, 0x80, 0x80, 0x80, 0xF0], },
                                 Sprite { rows: [0xE0, 0x90, 0x90, 0x90, 0xE0], },
                                 Sprite { rows: [0xF0, 0x80, 0xF0, 0x80, 0xF0], },
                                 Sprite { rows: [0xF0, 0x80, 0xF0, 0x80, 0x80], },];
const DISPLAY_WIDTH: usize = 8;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    bytes: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            bytes: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn draw_sprite(&mut self, point: Point, sprite: usize) {
        assert!(sprite < 0x10);
        assert!(point.1 + 5 < DISPLAY_HEIGHT as u8);
        assert!(point.0 < DISPLAY_WIDTH as u8 * 8);

        let enumeration = SPRITES[sprite].rows.iter().enumerate();
        if point.0 % 8 == 0 {
            let col = point.0 / 8;
            for (index, row) in enumeration {
                self.bytes[point.1 as usize + index][col as usize] ^= *row;
            }
        } else {
            let col = point.0 / 8;
            let first_offset = point.0 % 8;
            let last_offset = 8 - first_offset;
            for (index, row) in enumeration {
                let first_byte = *row >> first_offset;
                let last_byte = *row << last_offset;
                self.bytes[point.1 as usize + index][col as usize] ^= first_byte;
                self.bytes[point.1 as usize + index][col as usize + 1] ^= last_byte;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        for row in self.bytes.iter_mut() {
            for col in row.iter_mut() {
                *col = 0;
            }
        }
    }

    pub fn flip(&self) {
        println!("{0}[2J{0}[H{1}", 27 as char, self);
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.bytes.iter() {
            for col in row.iter() {
                for idx in 0..8 {
                    let bool = ((col >> 7 - idx) & 0x1u8) == 1u8;
                    write!(f, "{}", if bool { '#' } else { ' ' })?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
