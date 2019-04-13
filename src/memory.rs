pub const MEMORY_SIZE: usize = 0xFFF;

pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
    pub size: usize,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bytes: [0; MEMORY_SIZE],
            size: MEMORY_SIZE,
        }
    }

    pub fn write(&mut self, address: usize, to_write: &[u8]) {
        assert!(is_valid_address(address + to_write.len()));
        for (index, byte) in to_write.iter().enumerate()
        {
            self.bytes[address + index] = *byte;
        }
    }

    pub fn read(&self, address: usize, buffer: &mut [u8]) {
        assert!(is_valid_address(address + buffer.len()));
        for (index, byte) in self.bytes[address..address+buffer.len()]
            .iter().enumerate()
        {
            buffer[index] = *byte;
        }
    }
}

pub fn is_valid_address(address: usize) -> bool {
    address < MEMORY_SIZE
}
