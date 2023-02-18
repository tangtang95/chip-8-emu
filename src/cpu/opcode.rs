#[derive(Debug, PartialEq)]
pub enum Opcode {
    MachineLanguageRoutine(u16),
    Clear,
    SubroutineReturn,
    SubroutineCall(usize),
    Jump(usize),
    SkipEqVal(u8, u8),
    SkipNotEqVal(u8, u8),
    SkipEqReg(u8, u8),
    SkipNotEqReg(u8, u8),
    SetValueToRegister(u8, u8),
    AddValueToRegister(u8, u8),
    CopyRegister(u8, u8),
    BinaryOR(u8, u8),
    BinaryAND(u8, u8),
    BinaryXOR(u8, u8),
    AddRegister(u8, u8),
    SubtractRegister(u8, u8),
    NegativeSubtractRegister(u8, u8),
    ShiftRegisterLeft(u8, u8),
    ShiftRegisterRight(u8, u8),
    SetIndexRegister(usize),
    JumpOffset(usize),
    Random(u8, u8),
    Display(u8, u8, u8),
    SkipIfKeyPressed(u8),
    SkipIfKeyNotPressed(u8),
    CopyDelayTimerValue(u8),
    SetDelayTimer(u8),
    SetSoundTimer(u8),
    AddIndexRegister(u8),
    GetKey(u8),
    FontCharacter(u8),
    DecimalConversion(u8),
    StoreMemory(u8),
    LoadMemory(u8),
    Unknown
}

impl From<u16> for Opcode {
    fn from(instruction: u16) -> Self {
        let third_half_byte: u8 = ((instruction & 0x00F0) >> 4) as u8; // 00X0 -> X
        let second_half_byte: u8 = ((instruction & 0x0F00) >> 8) as u8; // 0X00 -> X 
        let last_half_byte: u8 = (instruction & 0x000F) as u8; // 000X -> X
        let last_single_byte: u8 = (instruction & 0x00FF) as u8; // 00XX -> XX
        let all_data: u16 = instruction & 0x0FFF; // 0XXX -> XXX

        match (instruction >> 12) as u8 {
            0x0 => match all_data {
                0x0E0 => Opcode::Clear,
                0x0EE => Opcode::SubroutineReturn,
                _ => Opcode::MachineLanguageRoutine(all_data)
            },
            0x1 => Opcode::Jump(all_data as usize),
            0x2 => Opcode::SubroutineCall(all_data as usize),
            0x3 => Opcode::SkipEqVal(second_half_byte, last_single_byte),
            0x4 => Opcode::SkipNotEqVal(second_half_byte, last_single_byte),
            0x5 => Opcode::SkipEqReg(second_half_byte, third_half_byte),
            0x6 => Opcode::SetValueToRegister(second_half_byte, last_single_byte),
            0x7 => Opcode::AddValueToRegister(second_half_byte, last_single_byte),
            0x8 => match last_half_byte {
                    0x0 => Opcode::CopyRegister(second_half_byte, third_half_byte),
                    0x1 => Opcode::BinaryOR(second_half_byte, third_half_byte),
                    0x2 => Opcode::BinaryAND(second_half_byte, third_half_byte),
                    0x3 => Opcode::BinaryXOR(second_half_byte, third_half_byte),
                    0x4 => Opcode::AddRegister(second_half_byte, third_half_byte),
                    0x5 => Opcode::SubtractRegister(second_half_byte, third_half_byte),
                    0x6 => Opcode::ShiftRegisterRight(second_half_byte, third_half_byte),
                    0x7 => Opcode::NegativeSubtractRegister(second_half_byte, third_half_byte),
                    0xE => Opcode::ShiftRegisterLeft(second_half_byte, third_half_byte),
                    _ => Opcode::Unknown
                },
            0x9 => Opcode::SkipNotEqReg(second_half_byte, third_half_byte),
            0xA => Opcode::SetIndexRegister(all_data as usize),
            0xB => Opcode::JumpOffset(all_data as usize),
            0xC => Opcode::Random(second_half_byte, last_single_byte),
            0xD => Opcode::Display(second_half_byte, third_half_byte, last_half_byte),
            0xE => match last_single_byte {
                0x9E => Opcode::SkipIfKeyPressed(second_half_byte),
                0xA1 => Opcode::SkipIfKeyNotPressed(second_half_byte),
                _ => Opcode::Unknown
            },
            0xF => match last_single_byte {
                0x07 => Opcode::CopyDelayTimerValue(second_half_byte),
                0x15 => Opcode::SetDelayTimer(second_half_byte),
                0x18 => Opcode::SetSoundTimer(second_half_byte),
                0x1E => Opcode::AddIndexRegister(second_half_byte),
                0x0A => Opcode::GetKey(second_half_byte),
                0x29 => Opcode::FontCharacter(second_half_byte),
                0x33 => Opcode::DecimalConversion(second_half_byte),
                0x55 => Opcode::StoreMemory(second_half_byte),
                0x65 => Opcode::LoadMemory(second_half_byte),
                _ => Opcode::Unknown
            }
            _ => Opcode::Unknown
        }
    }
}

#[cfg(test)]
mod test {
    use super::Opcode;

    #[test]
    fn test_opcode_clear() {
        let opcode = Opcode::from(0x00E0);
        
        assert_eq!(opcode, Opcode::Clear);
    }

    #[test]
    fn test_opcode_add_value_to_register() {
        let opcode = Opcode::from(0x7050);
        
        assert_eq!(opcode, Opcode::AddValueToRegister(0, 0x50));
    }
}