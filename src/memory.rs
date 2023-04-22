pub struct Memory {
    data: [u8; 4096]
}

impl Memory {
    pub const ROM_INIT_ADDRESS: usize = 0x200;
    const FONT_INIT_ADDRESS: usize = 0x50;
    const FONT_WIDTH: u16 = 5;
    const FONT_DATA: [u8; 80] = [
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
    ];

    pub fn new() -> Self {
        Self { data: [0; 4096] }
    }

    pub fn load_font_data(&mut self) {
        self.data[Memory::FONT_INIT_ADDRESS..(Memory::FONT_INIT_ADDRESS + Memory::FONT_DATA.len())].copy_from_slice(&Memory::FONT_DATA);
    }

    pub fn load_rom_data(&mut self, rom_data: &[u8]) {
        self.data[Memory::ROM_INIT_ADDRESS..(Memory::ROM_INIT_ADDRESS + rom_data.len())].copy_from_slice(rom_data);
    }

    pub fn read_instruction(&self, pc: usize) -> u16 {
        (self.data[pc] as u16) << 8  | self.data[pc + 1] as u16
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }
    
    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn get_font_address(&self, font_idx: u8) -> usize {
        Memory::FONT_INIT_ADDRESS + (Memory::FONT_WIDTH * font_idx as u16) as usize
    }

}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
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