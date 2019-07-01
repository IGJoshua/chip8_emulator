use crate::sprite::Sprite;
use processing;
use processing::backend::glutin::glutin::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent,
};
use processing::errors::ProcessingErr;
use processing::shapes::rect::Rect;
use processing::Screen;
use std::fmt;

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
                let row_idx = (point.1 as usize + index) % DISPLAY_HEIGHT;
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
                let row_idx = (point.1 as usize + index) % DISPLAY_HEIGHT;

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

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

const COL_SIZE: f64 = (SCREEN_WIDTH as f64 / (DISPLAY_WIDTH * 8) as f64) / SCREEN_WIDTH as f64;
const ROW_SIZE: f64 = (SCREEN_HEIGHT as f64 / DISPLAY_HEIGHT as f64) / SCREEN_HEIGHT as f64;

const NUM_KEYS: usize = 0x10;

struct Key(processing::Key, bool);

struct Keyboard {
    keys: [Key; NUM_KEYS],
}

pub struct Window<'a> {
    screen: Screen<'a>,
    rect: Rect<'a>,
    keyboard: Keyboard,
}

impl<'a> Window<'a> {
    pub fn new() -> Window<'a> {
        let mut screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT, false, true, true).unwrap();

        screen.fill(&[1.], &[1.], &[1.], &[1.]);
        screen.fill_on();
        screen.stroke_off();
        screen.background(0., 0., 0., 1.);

        let rect = Rect::new(
            &screen,
            &[0.],
            &[0.],
            &[0.],
            &[COL_SIZE * 2.1],
            &[ROW_SIZE * 2.2],
        )
        .unwrap();

        let keyboard = Keyboard {
            keys: [
                Key(processing::Key::Num1, false),
                Key(processing::Key::Num2, false),
                Key(processing::Key::Num3, false),
                Key(processing::Key::Q, false),
                Key(processing::Key::W, false),
                Key(processing::Key::E, false),
                Key(processing::Key::A, false),
                Key(processing::Key::S, false),
                Key(processing::Key::D, false),
                Key(processing::Key::X, false),
                Key(processing::Key::Z, false),
                Key(processing::Key::C, false),
                Key(processing::Key::Num4, false),
                Key(processing::Key::R, false),
                Key(processing::Key::F, false),
                Key(processing::Key::V, false),
            ],
        };

        Window {
            screen,
            rect,
            keyboard,
        }
    }

    pub fn draw_display(&mut self, display: &Display) -> Result<Vec<Event>, ProcessingErr> {
        let screen = &mut self.screen;
        let rect = &self.rect;

        screen.reset_matrix();
        screen.translate(
            -(COL_SIZE * DISPLAY_WIDTH as f64 * 8.) as f32,
            -(ROW_SIZE * DISPLAY_HEIGHT as f64) as f32,
            0.,
        );
        for (row_idx, row) in display.bytes.iter().rev().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                for idx in 0..8 {
                    screen.translate(COL_SIZE as f32 * 2., 0., 0.);
                    let bool = ((col >> 7 - idx) & 0x1u8) == 1u8;
                    if bool {
                        screen.draw(&self.rect)?;
                    }
                }
            }
            screen.translate(
                -(COL_SIZE * DISPLAY_WIDTH as f64 * 8.) as f32 * 2.,
                ROW_SIZE as f32 * 2.,
                0.,
            );
        }

        screen.reveal_with_events()
    }

    pub fn process_events(&mut self, events: Vec<Event>) {
        let screen = &mut self.screen;

        for event in events {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        ..
                    } => {
                        let state = match state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        };

                        for key in self.keyboard.keys.iter_mut() {
                            let keycode: VirtualKeyCode = key.0.into();
                            if keycode == virtual_keycode.unwrap() {
                                key.1 = state;
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn wait_for_key(&mut self) -> u8 {
        loop {
            for (idx, Key(key, ..)) in self.keyboard.keys.iter().enumerate() {
                if self.screen.key_press(*key) {
                    return idx as u8;
                }
            }
            self.screen.poll_events();
        }
    }

    pub fn key_down(&self, key: u8) -> bool {
        self.keyboard.keys[key as usize].1
    }
}
