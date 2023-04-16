use rand::Rng;
use super::Cpu;

pub trait ShiftBehavior<'a, T> : Clone {
    fn opcode_shift_left_register(&self, cpu: &mut Cpu<'a, T>, reg_idx_x: u8, reg_idx_y: u8);
    fn opcode_shift_right_register(&self, cpu: &mut Cpu<'a, T>, reg_idx_x: u8, reg_idx_y: u8);
}

fn default_opcode_shift_left_register<'a, T: ShiftBehavior<'a, T>>(cpu: &mut Cpu<'a, T>, reg_idx_x: u8) {
    let x = cpu.var_regs[reg_idx_x as usize];
    cpu.set_flag_register((x & (1 << 7) > 0) as u8);
    cpu.var_regs[reg_idx_x as usize] = x << 1;
}

fn default_opcode_shift_right_register<'a, T: ShiftBehavior<'a, T>>(cpu: &mut Cpu<'a, T>, reg_idx_x: u8) {
    let x = cpu.var_regs[reg_idx_x as usize];
    cpu.set_flag_register(x & 0x1);
    cpu.var_regs[reg_idx_x as usize] = x >> 1;
}

#[derive(Clone)]
pub struct OldShiftBehavior;
impl<'a> ShiftBehavior<'a, OldShiftBehavior> for OldShiftBehavior {
    fn opcode_shift_left_register(&self, cpu: &mut Cpu<'a, OldShiftBehavior>, reg_idx_x: u8, _: u8) {
        default_opcode_shift_left_register(cpu, reg_idx_x);
    }

    fn opcode_shift_right_register(&self, cpu: &mut Cpu<'a, OldShiftBehavior>, reg_idx_x: u8, _: u8) {
        default_opcode_shift_right_register(cpu, reg_idx_x);
    }
}

#[derive(Clone)]
pub struct NewShiftBehavior;
impl<'a> ShiftBehavior<'a, NewShiftBehavior> for NewShiftBehavior {
    fn opcode_shift_left_register(&self, cpu: &mut Cpu<'a, NewShiftBehavior>, reg_idx_x: u8, reg_idx_y: u8) {
        cpu.var_regs[reg_idx_x as usize] = cpu.var_regs[reg_idx_y as usize];
        default_opcode_shift_left_register(cpu, reg_idx_x);
    }

    fn opcode_shift_right_register(&self, cpu: &mut Cpu<'a, NewShiftBehavior>, reg_idx_x: u8, reg_idx_y: u8) {
        cpu.var_regs[reg_idx_x as usize] = cpu.var_regs[reg_idx_y as usize];
        default_opcode_shift_right_register(cpu, reg_idx_x);
    }
}

impl<'a, T> Cpu<'a, T>
where T: ShiftBehavior<'a, T> {
    pub(super) fn opcode_clear(&mut self) {
        self.display.fill([0; 64]);
    }

    pub(super) fn opcode_jump(&mut self, address: usize) {
        self.pc = address;
    }

    pub(super) fn opcode_subroutine_call(&mut self, address: usize) {
        self.stack.push(self.pc as u16);
        self.opcode_jump(address);
    }

    pub(super) fn opcode_subroutine_return(&mut self) {
        if let Some(address) = self.stack.pop() {
            self.pc = address as usize;
        } else {
            panic!("Stack is empty!");
        }
    }

    pub(super) fn opcode_set_value_to_register(&mut self, reg_idx: u8, value: u8) {
        self.var_regs[reg_idx as usize] = value;
    }

    pub(super) fn opcode_add_value_to_register(&mut self, reg_idx: u8, value: u8) {
        self.var_regs[reg_idx as usize] = self.var_regs[reg_idx as usize].wrapping_add(value);
    }

    pub(super) fn opcode_add_registers(&mut self, reg_idx_x: u8, reg_idx_y: u8) {
        let x = self.var_regs[reg_idx_x as usize];
        let y = self.var_regs[reg_idx_y as usize];
        let (result, overflowed) = x.overflowing_add(y);
        self.set_flag_register(overflowed as u8);
        self.var_regs[reg_idx_x as usize] = result;
    }

    pub(super) fn opcode_subtract_registers(&mut self, dest_reg_idx: u8, reg_idx_x: u8, reg_idx_y: u8) {
        let x = self.var_regs[reg_idx_x as usize];
        let y = self.var_regs[reg_idx_y as usize];
        let (result, overflowed) = x.overflowing_sub(y);
        self.set_flag_register(!overflowed as u8);
        self.var_regs[dest_reg_idx as usize] = result;
    }

    pub(super) fn opcode_shift_left_register(&mut self, reg_idx_x: u8, reg_idx_y: u8) {
        self.shift_logic.clone().opcode_shift_left_register(self, reg_idx_x, reg_idx_y);
    }

    pub(super) fn opcode_shift_right_register(&mut self, reg_idx_x: u8, reg_idx_y: u8) {
        self.shift_logic.clone().opcode_shift_right_register(self, reg_idx_x, reg_idx_y);
    }

    pub(super) fn opcode_set_index_register(&mut self, value: usize) {
        self.index_reg = value;
    }

    pub(super) fn opcode_add_index_register(&mut self, reg_idx: u8) {
        let offset: usize = self.var_regs[reg_idx as usize] as usize;
        if self.index_reg + offset > 0x1000 { self.set_flag_register(1) }
        self.index_reg += offset;
    }

    pub(super) fn opcode_jump_with_offset(&mut self, address: usize) {
        // TODO: ambiguous instruction add configurability
        self.pc = address + self.var_regs[0] as usize;
    }

    pub(super) fn opcode_random(&mut self, reg_idx: u8, mask: u8) {
        let mut rng = rand::thread_rng();
        self.var_regs[reg_idx as usize] = rng.gen_range(0..=0xFF) & mask;
    }
    
    pub(super) fn opcode_set_index_register_to_font(&mut self, reg_idx: u8) {
        self.index_reg = self.memory.get_font_address(self.var_regs[reg_idx as usize] & 0x0F);
    }

    pub(super) fn opcode_display(&mut self, reg_idx_x: u8, reg_idx_y: u8, n_pixels: u8) {
        let initial_x = (self.var_regs[reg_idx_x as usize] as usize) % self.display[0].len();
        let mut y = (self.var_regs[reg_idx_y as usize] as usize) % self.display.len();

        self.set_flag_register(0);

        for i in 0..n_pixels {
            let mut x = initial_x;
            let sprite_byte = self.memory.read_byte(self.index_reg + i as usize);
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

    pub(super) fn opcode_skip_if_key_pressed(&mut self, reg_idx: u8) {
        let key = self.var_regs[reg_idx as usize] & 0xF;
        if self.input_state[key as usize] > 0 {
            self.next_opcode();
        }
    }

    pub(super) fn opcode_skip_if_key_not_pressed(&mut self, reg_idx: u8) {
        let key = self.var_regs[reg_idx as usize] & 0xF;
        if self.input_state[key as usize] == 0 {
            self.next_opcode();
        }
    }

    pub(super) fn opcode_get_key(&mut self, reg_idx: u8) {
        let key_action_state: Vec<i8> = self.input_state.iter().zip(self.last_input_state.iter())
            .map(|(&curr, &prev)| curr as i8 - prev as i8).collect();

        let key_released = key_action_state.iter().enumerate()
            .find(|(_, &key_state)| key_state < 0);

        match key_released {
            Some((key_idx, _)) => self.var_regs[reg_idx as usize] = key_idx as u8,
            None => self.prev_opcode()
        }
    }

    pub(super) fn opcode_apply_decimal_conversion(&mut self, reg_idx: u8) {
        let value = self.var_regs[reg_idx as usize];
        value.to_string().chars().enumerate()
            .map(|(i, d)| (i, d.to_digit(10).unwrap() as u8))
            .for_each(|(i, d)| self.memory.write_byte(self.index_reg + i, d));
    }

    /**
    This is the implementation of opcode 0xFX55, which is an ambiguous instruction since there are different implementation for interpreters.
    
    The implemented one comes from modern interpreters where the index register is not mutated
     */
    pub(super) fn opcode_store_memory(&mut self, last_reg_idx: u8) {
        (0..=last_reg_idx as usize)
            .for_each(|i| self.memory.write_byte(self.index_reg + i, self.var_regs[i]));
    }

    /**
    This is the implementation of opcode 0xFX65, which is an ambiguous instruction since there are different implementation for interpreters.
     
    The implemented one comes from modern interpreters where the index register is not mutated
     */
    pub(super) fn opcode_load_memory(&mut self, last_reg_idx: u8) {
        (0..=last_reg_idx as usize)
            .for_each(|i| self.var_regs[i] = self.memory.read_byte(self.index_reg + i));
    }
}
