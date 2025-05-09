pub mod instructions;
use crate::instructions::*;

use std::ops::{Index, IndexMut};

pub type Byte = u8;
pub type SByte = i8;
pub type Word = u16;

const MAX_MEM: usize = 1024 * 64;

pub struct Memory {
    pub data: [Byte; MAX_MEM],
}

impl Memory {
    pub fn initialise(&mut self) {
        self.data = [0; MAX_MEM];
    }

    pub fn set_values(&mut self, value: Byte) {
        for x in self.data.iter_mut() {
            *x = value;
        }
    }

    pub fn get(&self, addr: usize) -> Byte {
        self.data[addr]
    }

    pub fn set(&mut self, addr: usize, value: Byte) {
        self.data[addr] = value;
    }
}

impl Index<usize> for Memory {
    type Output = Byte;
    fn index(&self, addr: usize) -> &Self::Output {
        &self.data[addr]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, addr: usize) -> &mut Self::Output {
        &mut self.data[addr]
    }
}

#[derive(Default)]
pub struct StatusFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
    pub unused: bool,
}

pub struct Cpu {
    pub reg_a: Byte,
    pub reg_x: Byte,
    pub reg_y: Byte,
    pub status: StatusFlags,
    pub pc: Word,
    pub sp: Byte,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: StatusFlags::default(),
            pc: 0xFFFC,
            sp: 0xFD,
        }
    }

    pub fn reset(&mut self, memory: &mut Memory) {
        self.reset_vec(memory);
    }

    pub fn reset_vec(&mut self, memory: &mut Memory) {
        self.pc = ((memory.data[0xFFFD] as Word) << 8) | (memory.data[0xFFFC] as Word);
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.status = StatusFlags::default();
        // memory.set_values(0);
    }

    pub fn fetch_byte(&mut self, memory: &Memory, cycles: &mut i32) -> Byte {
        let data = memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        *cycles -= 1;
        data
    }

    pub fn fetch_sbyte(&mut self, memory: &Memory, cycles: &mut i32) -> SByte {
        self.fetch_byte(memory, cycles) as SByte
    }

    pub fn fetch_word(&mut self, memory: &Memory, cycles: &mut i32) -> Word {
        let lo = self.fetch_byte(memory, cycles) as Word;
        let hi = self.fetch_byte(memory, cycles) as Word;
        (hi << 8) | lo
    }

    pub fn read_byte(&self, memory: &Memory, address: Word, cycles: &mut i32) -> Byte {
        *cycles -= 1;
        memory[address as usize]
    }

    pub fn read_word(&self, memory: &Memory, address: Word, cycles: &mut i32) -> Word {
        *cycles -= 2;
        let lo = memory[address as usize] as Word;
        let hi = memory[address.wrapping_add(1) as usize] as Word;
        (hi << 8) | lo
    }

    pub fn write_byte(&self, memory: &mut Memory, address: Word, value: Byte, cycles: &mut i32) {
        *cycles -= 1;
        memory[address as usize] = value;
    }

    pub fn write_word(&self, memory: &mut Memory, address: Word, value: Word, cycles: &mut i32) {
        *cycles -= 2;
        memory[address as usize] = (value & 0xFF) as Byte;
        memory[address.wrapping_add(1) as usize] = (value >> 8) as Byte;
    }

    pub fn sp_to_address(&self) -> Word {
        0x100 | (self.sp as Word)
    }

    pub fn push_word_to_stack(&mut self, memory: &mut Memory, value: Word, cycles: &mut i32) {
        self.write_byte(memory, self.sp_to_address(), (value >> 8) as Byte, cycles);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(memory, self.sp_to_address(), (value & 0xFF) as Byte, cycles);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_pc_to_stack(&mut self, memory: &mut Memory, cycles: &mut i32) {
        self.push_word_to_stack(memory, self.pc, cycles);
    }

    // Addressing Modes
    fn addr_zero_page(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let zp_addr = self.fetch_byte(memory, cycles);
        zp_addr as Word
    }

    fn addr_absolute(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        self.fetch_word(memory, cycles)
    }
}

impl Cpu {
    /// Zero Page,X
    pub fn addr_zero_page_x(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let mut zp_addr = self.fetch_byte(memory, cycles);
        zp_addr = zp_addr.wrapping_add(self.reg_x);
        *cycles -= 1;
        zp_addr as Word
    }

    /// Zero Page,Y
    pub fn addr_zero_page_y(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let mut zp_addr = self.fetch_byte(memory, cycles);
        zp_addr = zp_addr.wrapping_add(self.reg_y);
        *cycles -= 1;
        zp_addr as Word
    }

    /// Absolute,X (with page boundary penalty)
    pub fn addr_absolute_x(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let base = self.fetch_word(memory, cycles);
        let addr = base.wrapping_add(self.reg_x as Word);
        if (base & 0xFF00) != (addr & 0xFF00) {
            *cycles -= 1;
        }
        addr
    }

    /// Absolute,X (always subtracts a cycle, for store instructions)
    pub fn addr_absolute_x_5(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let base = self.fetch_word(memory, cycles);
        let addr = base.wrapping_add(self.reg_x as Word);
        *cycles -= 1;
        addr
    }

    /// Absolute,Y (with page boundary penalty)
    pub fn addr_absolute_y(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let base = self.fetch_word(memory, cycles);
        let addr = base.wrapping_add(self.reg_y as Word);
        if (base & 0xFF00) != (addr & 0xFF00) {
            *cycles -= 1;
        }
        addr
    }

    /// Absolute,Y (always subtracts a cycle, for store instructions)
    pub fn addr_absolute_y_5(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let base = self.fetch_word(memory, cycles);
        let addr = base.wrapping_add(self.reg_y as Word);
        *cycles -= 1;
        addr
    }

    /// (Indirect,X)
    pub fn addr_indirect_x(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let mut zp_addr = self.fetch_byte(memory, cycles);
        zp_addr = zp_addr.wrapping_add(self.reg_x);
        *cycles -= 1;
        let lo = memory[zp_addr as usize] as Word;
        let hi = memory[zp_addr.wrapping_add(1) as usize] as Word;
        (hi << 8) | lo
    }

    /// (Indirect),Y (with page boundary penalty)
    pub fn addr_indirect_y(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let zp_addr = self.fetch_byte(memory, cycles);
        let lo = memory[zp_addr as usize] as Word;
        let hi = memory[zp_addr.wrapping_add(1) as usize] as Word;
        let base = (hi << 8) | lo;
        let addr = base.wrapping_add(self.reg_y as Word);
        if (base & 0xFF00) != (addr & 0xFF00) {
            *cycles -= 1;
        }
        addr
    }

    /// (Indirect),Y (always subtracts a cycle, for store instructions)
    pub fn addr_indirect_y_6(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let zp_addr = self.fetch_byte(memory, cycles);
        let lo = memory[zp_addr as usize] as Word;
        let hi = memory[zp_addr.wrapping_add(1) as usize] as Word;
        let base = (hi << 8) | lo;
        let addr = base.wrapping_add(self.reg_y as Word);
        *cycles -= 1;
        addr
    }

    /// JMP (indirect) – emulate 6502 bug: if the indirect address ends in 0xFF, the high byte wraps within the same page.
    pub fn addr_indirect_mp(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let ptr = self.fetch_word(memory, cycles);
        let lo = memory[ptr as usize] as Word;
        let hi_addr = if (ptr & 0x00FF) == 0x00FF {
            ptr & 0xFF00
        } else {
            ptr.wrapping_add(1)
        };
        let hi = memory[hi_addr as usize] as Word;
        (hi << 8) | lo
    }

    pub fn set_zero_and_negative_flags(&mut self, value: Byte) {
        self.status.zero = value == 0;
        self.status.negative = (value & 0x80) != 0;
    }
}

impl Cpu {
    // Helper functions for execute and operations
    fn load_register_a(&mut self, addr: Word, memory: &Memory, cycles: &mut i32) {
        self.reg_a = self.read_byte(memory, addr, cycles);
        self.set_zero_and_negative_flags(self.reg_a);
    }
    fn load_register_x(&mut self, addr: Word, memory: &Memory, cycles: &mut i32) {
        self.reg_x = self.read_byte(memory, addr, cycles);
        self.set_zero_and_negative_flags(self.reg_x);
    }
    fn load_register_y(&mut self, addr: Word, memory: &Memory, cycles: &mut i32) {
        self.reg_y = self.read_byte(memory, addr, cycles);
        self.set_zero_and_negative_flags(self.reg_y);
    }

    fn and(&mut self, addr: Word, memory: &Memory, cycles: &mut i32) {
        self.reg_a &= self.read_byte(memory, addr, cycles);
        self.set_zero_and_negative_flags(self.reg_a);
    }

    fn ora(&mut self, addr: Word, memory: &Memory, cycles: &mut i32) {
        self.reg_a |= self.read_byte(memory, addr, cycles);
        self.set_zero_and_negative_flags(self.reg_a);
    }

    fn eor(&mut self, addr: Word, memory: &Memory, cycles: &mut i32) {
        self.reg_a ^= self.read_byte(memory, addr, cycles);
        self.set_zero_and_negative_flags(self.reg_a);
    }

    // fn branch_if(&mut self, flag: bool, expected: bool, memory: &Memory, cycles: &mut i32) {
    //     let offset = self.fetch_sbyte(memory, cycles);
    //     if flag == expected {
    //         let old_pc = self.pc;
    //         self.pc = self.pc.wrapping_add(offset as Word);
    //         *cycles -= 1;
    //         if (old_pc & 0xFF00) != (self.pc & 0xFF00) {
    //             *cycles -= 1;
    //         }
    //     }
    // }

    fn adc(&mut self, operand: Byte) {
        let carry = if self.status.carry { 1 } else { 0 };
        let sum = self.reg_a as u16 + operand as u16 + carry as u16;
        let result = sum as Byte;
        self.status.carry = sum > 0xFF;
        self.status.overflow =
            ((self.reg_a ^ result) & (operand ^ result) & 0x80) != 0;
        self.reg_a = result;
        self.set_zero_and_negative_flags(self.reg_a);
    }

    fn sbc(&mut self, operand: Byte) {
        // 6502 SBC is ADC with the ones complement of the operand.
        self.adc(!operand);
    }

    fn cmp(&mut self, operand: Byte, reg: Byte) {
        let result = reg.wrapping_sub(operand);
        self.status.carry = reg >= operand;
        self.status.zero = result == 0;
        self.status.negative = (result & 0x80) != 0;
    }

    fn asl(&mut self, operand: Byte, cycles: &mut i32) -> Byte {
        self.status.carry = (operand & 0x80) != 0;
        let result = operand << 1;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }

    fn lsr(&mut self, operand: Byte, cycles: &mut i32) -> Byte {
        self.status.carry = (operand & 0x01) != 0;
        let result = operand >> 1;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }

    fn rol(&mut self, operand: Byte, cycles: &mut i32) -> Byte {
        let carry_in = if self.status.carry { 1 } else { 0 };
        self.status.carry = (operand & 0x80) != 0;
        let result = (operand << 1) | carry_in;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }

    fn ror(&mut self, operand: Byte, cycles: &mut i32) -> Byte {
        let carry_in = if self.status.carry { 0x80 } else { 0 };
        self.status.carry = (operand & 0x01) != 0;
        let result = (operand >> 1) | carry_in;
        self.set_zero_and_negative_flags(result);
        *cycles -= 1;
        result
    }
}

impl Cpu {
    /// Main execute loop – processes one opcode per loop.
    /// Returns the number of cycles consumed.
    pub fn execute(&mut self, mut cycles: i32, memory: &mut Memory) -> i32 {
        let cycles_requested = cycles;
        while cycles > 0 {
            let opcode = self.fetch_byte(memory, &mut cycles);
            match opcode {
                // --- Load Accumulator ---
                INS_LDA_IM => {
                    self.reg_a = self.fetch_byte(memory, &mut cycles);
                    self.set_zero_and_negative_flags(self.reg_a);
                }
                INS_LDA_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }
                INS_LDA_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }
                INS_LDA_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }
                INS_LDA_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }
                INS_LDA_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }
                INS_LDA_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }
                INS_LDA_INDY => {
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    self.load_register_a(addr, memory, &mut cycles);
                }

                // --- Load X/Y ---
                INS_LDX_IM => {
                    self.reg_x = self.fetch_byte(memory, &mut cycles);
                    self.set_zero_and_negative_flags(self.reg_x);
                }
                INS_LDX_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.load_register_x(addr, memory, &mut cycles);
                }
                INS_LDX_ZPY => {
                    let addr = self.addr_zero_page_y(&mut cycles, memory);
                    self.load_register_x(addr, memory, &mut cycles);
                }
                INS_LDX_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.load_register_x(addr, memory, &mut cycles);
                }
                INS_LDX_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    self.load_register_x(addr, memory, &mut cycles);
                }

                INS_LDY_IM => {
                    self.reg_y = self.fetch_byte(memory, &mut cycles);
                    self.set_zero_and_negative_flags(self.reg_y);
                }
                INS_LDY_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.load_register_y(addr, memory, &mut cycles);
                }
                INS_LDY_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.load_register_y(addr, memory, &mut cycles);
                }
                INS_LDY_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.load_register_y(addr, memory, &mut cycles);
                }
                INS_LDY_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    self.load_register_y(addr, memory, &mut cycles);
                }

                // --- Store Accumulator, X, Y ---
                INS_STA_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }
                INS_STA_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }
                INS_STA_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }
                INS_STA_ABSX => {
                    let addr = self.addr_absolute_x_5(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }
                INS_STA_ABSY => {
                    let addr = self.addr_absolute_y_5(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }
                INS_STA_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }
                INS_STA_INDY => {
                    let addr = self.addr_indirect_y_6(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_a, &mut cycles);
                }

                INS_STX_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_x, &mut cycles);
                }
                INS_STX_ZPY => {
                    let addr = self.addr_zero_page_y(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_x, &mut cycles);
                }
                INS_STX_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_x, &mut cycles);
                }

                INS_STY_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_y, &mut cycles);
                }
                INS_STY_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_y, &mut cycles);
                }
                INS_STY_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.write_byte(memory, addr, self.reg_y, &mut cycles);
                }

                // --- Transfer and Stack Operations ---
                INS_TSX => {
                    // Transfer stack pointer to X
                    self.reg_x = self.sp;
                    self.set_zero_and_negative_flags(self.reg_x);
                    cycles -= 1;
                }
                INS_TXS => {
                    // Transfer X to stack pointer
                    self.sp = self.reg_x;
                    cycles -= 1;
                }
                INS_PHA => {
                    self.push_word_to_stack(memory, self.reg_a as Word, &mut cycles);
                }
                INS_PLA => {
                    self.reg_a = self.read_byte(memory, self.sp_to_address(), &mut cycles);
                    self.set_zero_and_negative_flags(self.reg_a);
                }
                INS_PHP => {
                    // Push processor status (with unused bit set)
                    let mut status = 0;
                    if self.status.carry { status |= 0x01; }
                    if self.status.zero { status |= 0x02; }
                    if self.status.interrupt_disable { status |= 0x04; }
                    if self.status.decimal_mode { status |= 0x08; }
                    if self.status.break_command { status |= 0x10; }
                    if self.status.overflow { status |= 0x40; }
                    if self.status.negative { status |= 0x80; }
                    self.push_word_to_stack(memory, status as Word, &mut cycles);
                }
                INS_PLP => {
                    // Pull processor status from stack – not a full implementation
                    let status = self.read_byte(memory, self.sp_to_address(), &mut cycles);
                    self.status.carry = (status & 0x01) != 0;
                    self.status.zero = (status & 0x02) != 0;
                    self.status.interrupt_disable = (status & 0x04) != 0;
                    self.status.decimal_mode = (status & 0x08) != 0;
                    self.status.break_command = (status & 0x10) != 0;
                    self.status.overflow = (status & 0x40) != 0;
                    self.status.negative = (status & 0x80) != 0;
                }

                // --- Jumps and Calls ---
                INS_JMP_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.pc = addr;
                }
                INS_JMP_IND => {
                    let addr = self.addr_indirect_mp(&mut cycles, memory);
                    self.pc = addr;
                }
                INS_JSR => {
                    // Push (PC-1), then set PC to target
                    self.push_pc_to_stack(memory, &mut cycles);
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.pc = addr;
                }
                INS_RTS => {
                    // Pop return address and add one
                    let ret_addr = self.read_word(memory, self.sp_to_address(), &mut cycles);
                    self.pc = ret_addr.wrapping_add(1);
                }
                INS_BRK => {
                    self.status.break_command = true;
                    cycles = 0;
                }
                INS_RTI => {
                    // Pull processor status, then PC – simplified
                    let status = self.read_byte(memory, self.sp_to_address(), &mut cycles);
                    self.status.carry = (status & 0x01) != 0;
                    self.status.zero = (status & 0x02) != 0;
                    self.status.interrupt_disable = (status & 0x04) != 0;
                    self.status.decimal_mode = (status & 0x08) != 0;
                    self.status.break_command = (status & 0x10) != 0;
                    self.status.overflow = (status & 0x40) != 0;
                    self.status.negative = (status & 0x80) != 0;
                    self.pc = self.read_word(memory, self.sp_to_address(), &mut cycles);
                }

                // --- Logical Ops: AND, ORA, EOR, BIT ---
                INS_AND_IM => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.reg_a &= value;
                    self.set_zero_and_negative_flags(self.reg_a);
                }
                INS_AND_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }
                INS_AND_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }
                INS_AND_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }
                INS_AND_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }
                INS_AND_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }
                INS_AND_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }
                INS_AND_INDY => {
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    self.and(addr, memory, &mut cycles);
                }

                INS_ORA_IM => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.reg_a |= value;
                    self.set_zero_and_negative_flags(self.reg_a);
                }
                INS_ORA_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }
                INS_ORA_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }
                INS_ORA_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }
                INS_ORA_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }
                INS_ORA_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }
                INS_ORA_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }
                INS_ORA_INDY => {
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    self.ora(addr, memory, &mut cycles);
                }

                INS_EOR_IM => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.reg_a ^= value;
                    self.set_zero_and_negative_flags(self.reg_a);
                }
                INS_EOR_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }
                INS_EOR_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }
                INS_EOR_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }
                INS_EOR_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }
                INS_EOR_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }
                INS_EOR_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }
                INS_EOR_INDY => {
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    self.eor(addr, memory, &mut cycles);
                }

                INS_BIT_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let value = self.read_byte(memory, addr, &mut cycles);
                    self.status.zero = (self.reg_a & value) == 0;
                    self.status.negative = (value & 0x80) != 0;
                    self.status.overflow = (value & 0x40) != 0;
                }
                INS_BIT_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let value = self.read_byte(memory, addr, &mut cycles);
                    self.status.zero = (self.reg_a & value) == 0;
                    self.status.negative = (value & 0x80) != 0;
                    self.status.overflow = (value & 0x40) != 0;
                }

                // --- Arithmetic ---
                INS_ADC => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }
                INS_ADC_INDY => {
                    self.fetch_byte(memory, &mut cycles); // Dummy read for timing
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.adc(operand);
                }

                INS_SBC => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }
                INS_SBC_INDY => {
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.sbc(operand);
                }

                // --- Comparison ---
                INS_CMP => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_ABSY => {
                    let addr = self.addr_absolute_y(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_INDX => {
                    let addr = self.addr_indirect_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }
                INS_CMP_INDY => {
                    let addr = self.addr_indirect_y(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_a);
                }

                INS_CPX => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    self.cmp(operand, self.reg_x);
                }
                INS_CPX_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_x);
                }
                INS_CPX_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_x);
                }
                INS_CPY => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    self.cmp(operand, self.reg_y);
                }
                INS_CPY_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_y);
                }
                INS_CPY_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    self.cmp(operand, self.reg_y);
                }

                // --- Increments/Decrements ---
                INS_INX => {
                    self.reg_x = self.reg_x.wrapping_add(1);
                    self.set_zero_and_negative_flags(self.reg_x);
                    cycles -= 1;
                }
                INS_INY => {
                    self.reg_y = self.reg_y.wrapping_add(1);
                    self.set_zero_and_negative_flags(self.reg_y);
                    cycles -= 1;
                }
                INS_DEX => {
                    self.reg_x = self.reg_x.wrapping_sub(1);
                    self.set_zero_and_negative_flags(self.reg_x);
                    cycles -= 1;
                }
                INS_DEY => {
                    self.reg_y = self.reg_y.wrapping_sub(1);
                    self.set_zero_and_negative_flags(self.reg_y);
                    cycles -= 1;
                }
                INS_DEC_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_sub(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_DEC_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_sub(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_DEC_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_sub(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_DEC_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_sub(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_INC_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_add(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_INC_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_add(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_INC_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_add(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }
                INS_INC_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let tmp = self.read_byte(memory, addr, &mut cycles).wrapping_add(1);
                    self.write_byte(memory, addr, tmp, &mut cycles);
                    self.set_zero_and_negative_flags(tmp);
                }

                // --- Shifts ---
                INS_ASL => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    let result = self.asl(operand, &mut cycles);
                    // For accumulator mode, update A.
                    self.reg_a = result;
                }
                INS_ASL_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.asl(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ASL_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.asl(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ASL_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.asl(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ASL_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.asl(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }

                INS_LSR => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    let result = self.lsr(operand, &mut cycles);
                    self.reg_a = result;
                }
                INS_LSR_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.lsr(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_LSR_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.lsr(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_LSR_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.lsr(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_LSR_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.lsr(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }

                INS_ROL => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    let result = self.rol(operand, &mut cycles);
                    self.reg_a = result;
                }
                INS_ROL_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.rol(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ROL_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.rol(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ROL_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.rol(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ROL_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.rol(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }

                INS_ROR => {
                    let operand = self.fetch_byte(memory, &mut cycles);
                    let result = self.ror(operand, &mut cycles);
                    self.reg_a = result;
                }
                INS_ROR_ZP => {
                    let addr = self.addr_zero_page(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.ror(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ROR_ZPX => {
                    let addr = self.addr_zero_page_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.ror(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ROR_ABS => {
                    let addr = self.addr_absolute(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.ror(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }
                INS_ROR_ABSX => {
                    let addr = self.addr_absolute_x(&mut cycles, memory);
                    let operand = self.read_byte(memory, addr, &mut cycles);
                    let result = self.ror(operand, &mut cycles);
                    self.write_byte(memory, addr, result, &mut cycles);
                }

                // --- Flag and Status Changes ---
                INS_CLC => {
                    self.status.carry = false;
                    cycles -= 1;
                }
                INS_SEC => {
                    self.status.carry = true;
                    cycles -= 1;
                }
                INS_CLD => {
                    self.status.decimal_mode = false;
                    cycles -= 1;
                }
                INS_SED => {
                    self.status.decimal_mode = true;
                    cycles -= 1;
                }
                INS_CLI => {
                    self.status.interrupt_disable = false;
                    cycles -= 1;
                }
                INS_SEI => {
                    self.status.interrupt_disable = true;
                    cycles -= 1;
                }
                INS_CLV => {
                    self.status.overflow = false;
                    cycles -= 1;
                }

                // --- No Operation ---
                INS_NOP => {
                    cycles -= 1;
                }

                _ => {
                    panic!("Instruction {:02X} not implemented", opcode);
                }
            }
        }
        cycles_requested - cycles
    }
}
