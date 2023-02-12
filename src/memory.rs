pub struct Memory {
    data: [u8; 4096]
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0u8; 4096]}
    }

    pub fn load_font_data(&mut self) {
        self.data[0x50..=0x9F].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ]);
    }

    pub fn load_rom_data(&mut self, rom_data: &[u8]) {
        self.data[0x200..(0x200 + rom_data.len())].copy_from_slice(rom_data);
    }

    pub fn read_instruction(&self, pc: u16) -> u16 {
        (self.data[pc as usize] as u16) << 8  | self.data[pc as usize + 1] as u16
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

    #[test]
    fn memory_test() {
        let mut memory = Memory::new();
        memory.load_font_data();

        assert_eq!(memory.data[0x0], 0x00);
        assert_eq!(memory.data[0x50], 0xF0);
        assert_eq!(memory.data[0x51], 0x90);
        assert_eq!(memory.data[0x9F], 0x80);
    }
}