pub struct Sprite {
    pub rows: [u8; 5],
}

pub const SPRITES: [Sprite; 0x10] = [Sprite { rows: [0xF0, 0x90, 0x90, 0x90, 0xF0], },
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
                                     Sprite { rows: [0xF0, 0x80, 0xF0, 0x80, 0x80], }];
