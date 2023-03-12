pub mod opcode;
pub mod opcode_impl;

use crate::{memory::Memory, stack::Stack, timer::Timer};
use self::opcode::Opcode;

pub struct Cpu<'a> {
    pc: usize,
    index_reg: usize,
    var_regs: [u8; 16],
    stack: Stack<u16>,
    display: [[u8; 64]; 32],
    input_state: [u8; 16],
    last_input_state: [u8; 16],
    memory: &'a mut Memory,
    op_frequency: u32
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Cpu {
            pc: Memory::ROM_INIT_ADDRESS,
            index_reg: 0,
            var_regs: [0; 16],
            stack: Stack::new(),
            display: [[0; 64]; 32],
            input_state: [0; 16],
            last_input_state: [0; 16],
            memory,
            op_frequency: 700
        }
    }

    pub fn tick(&mut self, timer: &mut Timer) {
        let instruction = self.fetch();
        let opcode = self.decode(instruction);
        self.execute(opcode, timer);
    }

    pub fn update_input_state(&mut self, input_state: [u8; 16]) {
        self.last_input_state = self.input_state;
        self.input_state = input_state;
    }

    pub fn get_display(&self) -> &[[u8; 64]; 32] {
        &self.display
    }

    pub fn get_cpu_frequency(&self) -> u32 {
        self.op_frequency
    }

    fn fetch(&mut self) -> u16 {
        let instruction = self.memory.read_instruction(self.pc);
        self.next_opcode();

        instruction
    }

    fn decode(&self, instruction: u16) -> Opcode {
        Opcode::from(instruction)
    }

    fn execute(&mut self, opcode: Opcode, timer: &mut Timer) {
        match opcode {
            Opcode::MachineLanguageRoutine(_) => unimplemented!(),
            Opcode::Clear => self.opcode_clear(),
            Opcode::SubroutineReturn => self.opcode_subroutine_return(),
            Opcode::SubroutineCall(address) => self.opcode_subroutine_call(address),
            Opcode::Jump(address) => self.opcode_jump(address),
            Opcode::SkipEqVal(reg_idx, value) => if self.var_regs[reg_idx as usize] == value { self.next_opcode(); },
            Opcode::SkipNotEqVal(reg_idx, value) => if self.var_regs[reg_idx as usize] != value { self.next_opcode(); },
            Opcode::SkipEqReg(reg_idx_x, reg_idx_y) => if self.var_regs[reg_idx_x as usize] == self.var_regs[reg_idx_y as usize] { self.next_opcode(); },
            Opcode::SkipNotEqReg(reg_idx_x, reg_idx_y) => if self.var_regs[reg_idx_x as usize] != self.var_regs[reg_idx_y as usize] { self.next_opcode(); },
            Opcode::SetValueToRegister(reg_idx, value) => self.opcode_set_value_to_register(reg_idx, value),
            Opcode::AddValueToRegister(reg_idx, value) => self.opcode_add_value_to_register(reg_idx, value),
            Opcode::CopyRegister(reg_idx_x, reg_idx_y) => self.var_regs[reg_idx_x as usize] = self.var_regs[reg_idx_y as usize],
            Opcode::BinaryOR(reg_idx_x, reg_idx_y) => self.var_regs[reg_idx_x as usize] |= self.var_regs[reg_idx_y as usize],
            Opcode::BinaryAND(reg_idx_x, reg_idx_y) => self.var_regs[reg_idx_x as usize] &= self.var_regs[reg_idx_y as usize],
            Opcode::BinaryXOR(reg_idx_x, reg_idx_y) => self.var_regs[reg_idx_x as usize] ^= self.var_regs[reg_idx_y as usize],
            Opcode::AddRegister(reg_idx_x, reg_idx_y) => self.opcode_add_registers(reg_idx_x, reg_idx_y),
            Opcode::SubtractRegister(reg_idx_x, reg_idx_y) => self.opcode_subtract_registers(reg_idx_x, reg_idx_x, reg_idx_y),
            Opcode::NegativeSubtractRegister(reg_idx_x, reg_idx_y) => self.opcode_subtract_registers(reg_idx_x, reg_idx_y, reg_idx_x),
            Opcode::ShiftRegisterLeft(reg_idx_x, reg_idx_y) => self.opcode_shift_left_register(reg_idx_x, reg_idx_y),
            Opcode::ShiftRegisterRight(reg_idx_x, reg_idx_y) => self.opcode_shift_right_register(reg_idx_x, reg_idx_y),
            Opcode::SetIndexRegister(value) => self.opcode_set_index_register(value),
            Opcode::JumpOffset(address) => self.opcode_jump_with_offset(address),
            Opcode::Random(reg_idx, mask) => self.opcode_random(reg_idx, mask),
            Opcode::Display(reg_idx_x, reg_idx_y, n_pixels) => self.opcode_display(reg_idx_x, reg_idx_y, n_pixels),
            Opcode::SkipIfKeyPressed(reg_idx) => self.opcode_skip_if_key_pressed(reg_idx),
            Opcode::SkipIfKeyNotPressed(reg_idx) => self.opcode_skip_if_key_not_pressed(reg_idx),
            Opcode::CopyDelayTimerValue(reg_idx) => self.var_regs[reg_idx as usize] = timer.get_delay_timer(),
            Opcode::SetDelayTimer(reg_idx) => timer.set_delay_timer(self.var_regs[reg_idx as usize]),
            Opcode::SetSoundTimer(reg_idx) => timer.set_sound_timer(self.var_regs[reg_idx as usize]),
            Opcode::AddIndexRegister(reg_idx) => self.opcode_add_index_register(reg_idx),
            Opcode::GetKey(reg_idx) => self.opcode_get_key(reg_idx),
            Opcode::FontCharacter(reg_idx) => self.opcode_set_index_register_to_font(reg_idx),
            Opcode::DecimalConversion(reg_idx) => self.opcode_apply_decimal_conversion(reg_idx),
            Opcode::StoreMemory(last_reg_idx) => self.opcode_store_memory(last_reg_idx),
            Opcode::LoadMemory(last_reg_idx) => self.opcode_load_memory(last_reg_idx),
            Opcode::Unknown => panic!("Unknown opcode"),
        }
    }

    fn next_opcode(&mut self) {
        self.pc += 2;
    }

    fn prev_opcode(&mut self) {
        self.pc -= 2;
    }

    fn set_flag_register(&mut self, value: u8) {
        self.var_regs[0xF] = value;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn cpu_test() {

    }
}