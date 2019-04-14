use std::fmt;
use crate::sprite::Sprite;

#[derive(Clone, Copy)]
pub struct Point(pub u8, pub u8);

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

    pub fn draw_sprite(&mut self, point: Point, sprite: Sprite) -> bool {
        let mut res = false;

        let enumeration = sprite.rows.iter().enumerate();
        if point.0 % 8 == 0 {
            let col = point.0 as usize / 8;
            for (index, row) in enumeration {
                let row_idx =(point.1 as usize + index) % DISPLAY_HEIGHT;
                let byte = &mut self.bytes[row_idx][col];
                if *byte & row != 0 {
                    res = true;
                }
                *byte ^= *row;
            }
        } else {
            let col = point.0 / 8;
            let first_offset = point.0 % 8;
            let last_offset = 8 - first_offset;
            for (index, row) in enumeration {
                let row_idx =(point.1 as usize + index) % DISPLAY_HEIGHT;

                let first_byte = *row >> first_offset;
                let last_byte = *row << last_offset;

                let first_ref = &mut self.bytes[row_idx][col as usize];
                if *first_ref & first_byte != 0 {
                    res = true;
                }
                *first_ref ^= first_byte;

                let last_ref = &mut self.bytes[row_idx][(col as usize + 1) % DISPLAY_WIDTH];
                if *last_ref & last_byte != 0 {
                    res = true;
                }
                *last_ref ^= last_byte;
            }
        }
        res
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
