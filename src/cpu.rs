pub mod opcode;

use sdl2::{render::Canvas, video::Window, pixels::Color};
use crate::{memory::Memory, stack::Stack, timer::Timer};
use self::opcode::Opcode;

pub struct Cpu<'a> {
    pc: u16,
    index_reg: u16,
    var_regs: [u8; 16],
    stack: Stack<u16>,
    display: [[u8; 64]; 32],
    memory: &'a mut Memory,
    timer: &'a mut Timer
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &'a mut Memory, timer: &'a mut Timer) -> Self {
        Cpu {
            pc: 0x200,
            index_reg: 0,
            var_regs: [0u8; 16],
            stack: Stack::new(),
            display: [[0u8; 64]; 32],
            memory,
            timer
        }
    }

    pub fn update(&mut self) {
        let instruction = self.fetch();
        let opcode = self.decode(instruction);
        self.execute(opcode);
    }

    pub fn get_display(&self) -> &[[u8; 64]; 32] {
        &self.display
    }

    fn fetch(&mut self) -> u16 {
        let instruction = self.memory.read_instruction(self.pc);
        self.pc += 2;

        instruction
    }

    fn decode(&self, instruction: u16) -> Opcode {
        Opcode::from(instruction)
    }

    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::MachineLanguageRoutine(_) => unimplemented!(),
            Opcode::Clear => self.opcode_clear(),
            Opcode::SubroutineReturn => todo!(),
            Opcode::SubroutineCall(_) => todo!(),
            Opcode::Jump(address) => self.opcode_jump(address),
            Opcode::SkipEqVal(_, _) => todo!(),
            Opcode::SkipNotEqVal(_, _) => todo!(),
            Opcode::SkipEqReg(_, _) => todo!(),
            Opcode::SkipNotEqReg(_, _) => todo!(),
            Opcode::SetValueToRegister(reg_idx, value) => self.opcode_set_value_to_register(reg_idx, value),
            Opcode::AddValueToRegister(reg_idx, value) => self.opcode_add_value_to_register(reg_idx, value),
            Opcode::CopyRegister(_, _) => todo!(),
            Opcode::BinaryOR(_, _) => todo!(),
            Opcode::BinaryAND(_, _) => todo!(),
            Opcode::BinaryXOR(_, _) => todo!(),
            Opcode::AddRegister(_, _) => todo!(),
            Opcode::SubtractRegister(_, _) => todo!(),
            Opcode::NegativeSubtractRegister(_, _) => todo!(),
            Opcode::ShiftRegisterLeft(_, _) => todo!(),
            Opcode::ShiftRegisterRight(_, _) => todo!(),
            Opcode::SetIndexRegister(value) => self.opcode_set_index_register(value),
            Opcode::JumpOffset(_) => todo!(),
            Opcode::Random(_, _) => todo!(),
            Opcode::Display(reg_idx_x, reg_idx_y, n_pixels) => self.opcode_display(reg_idx_x, reg_idx_y, n_pixels),
            Opcode::SkipIfKeyPressed(_) => todo!(),
            Opcode::SkipIfKeyNotPressed(_) => todo!(),
            Opcode::CopyDelayTimerValue(_) => todo!(),
            Opcode::SetDelayTimer(_) => todo!(),
            Opcode::SetSoundTimer(_) => todo!(),
            Opcode::AddIndexRegister(_) => todo!(),
            Opcode::GetKey(_) => todo!(),
            Opcode::FontCharacter(_) => todo!(),
            Opcode::DecimalConversion(_) => todo!(),
            Opcode::StoreMemory(_) => todo!(),
            Opcode::LoadMemory(_) => todo!(),
            Opcode::Unknown => panic!("Unknown opcode"),
        }
    }

    fn set_flag_register(&mut self, value: u8) {
        self.var_regs[0xF] = value;
    }
}

/**
 * Opcodes implementation
 */
impl<'a> Cpu<'a> {
    fn opcode_clear(&mut self) {
        self.display.fill([0u8; 64]);
    }

    fn opcode_jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn opcode_set_value_to_register(&mut self, reg_idx: u8, value: u8) {
        self.var_regs[reg_idx as usize] = value;
    }

    fn opcode_add_value_to_register(&mut self, reg_idx: u8, value: u8) {
        self.var_regs[reg_idx as usize] += value;
    }

    fn opcode_set_index_register(&mut self, value: u16) {
        self.index_reg = value;
    }

    fn opcode_display(&mut self, reg_idx_x: u8, reg_idx_y: u8, n_pixels: u8) {
        let initial_x = (self.var_regs[reg_idx_x as usize] as usize) % self.display[0].len();
        let mut y = (self.var_regs[reg_idx_y as usize] as usize) % self.display.len();

        self.set_flag_register(0);

        for i in 0..n_pixels {
            let mut x = initial_x;
            let sprite_byte = self.memory.read_byte(self.index_reg + i as u16);
            for j in 0..8 {
                let bitmask = 1 << 7 - j;
                let sprite_bit_on = (sprite_byte & bitmask) > 0;
                if sprite_bit_on {
                    if self.display[y][x] > 0 {
                        self.display[y][x] = 0;
                        self.set_flag_register(1);
                    } else {
                        self.display[y][x] = 1;
                    }
                }

                x += 1;
                if x >= self.display[0].len() { break; }
            }
            y += 1;
            if y >= self.display.len() { break; }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn cpu_test() {

    }
}