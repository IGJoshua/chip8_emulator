struct Sprite {
    rows: [u8; 5],
}

struct Point(u8, u8);

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
}
