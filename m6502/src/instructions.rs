type Byte = u8;

// Process Status Bits
pub const NEGATIVE_FLAG_BIT: Byte = 0b10000000;
pub const OVERFLOW_FLAG_BIT: Byte = 0b01000000;
pub const BREAK_FLAG_BIT: Byte = 0b00001000;
pub const UNUSED_FLAG_BIT: Byte = 0b00010000;
pub const INTERRUPT_DISABLE_FLAG_BIT: Byte = 0b00000100;
pub const ZERO_FLAG_BIT: Byte = 0b00000001;

// opcodes
// LDA
pub const INS_LDA_IM: Byte = 0xA9;
pub const INS_LDA_ZP: Byte = 0xA5;
pub const INS_LDA_ZPX: Byte = 0xB5;
pub const INS_LDA_ABS: Byte = 0xAD;
pub const INS_LDA_ABSX: Byte = 0xBD;
pub const INS_LDA_ABSY: Byte = 0xB9;
pub const INS_LDA_INDX: Byte = 0xA1;
pub const INS_LDA_INDY: Byte = 0xB1;

// LDX
pub const INS_LDX_IM: Byte = 0xA2;
pub const INS_LDX_ZP: Byte = 0xA6;
pub const INS_LDX_ZPY: Byte = 0xB6;
pub const INS_LDX_ABS: Byte = 0xAE;
pub const INS_LDX_ABSY: Byte = 0xBE;

// LDY
pub const INS_LDY_IM: Byte = 0xA0;
pub const INS_LDY_ZP: Byte = 0xA4;
pub const INS_LDY_ZPX: Byte = 0xB4;
pub const INS_LDY_ABS: Byte = 0xAC;
pub const INS_LDY_ABSX: Byte = 0xBC;

// STA
pub const INS_STA_ZP: Byte = 0x85;
pub const INS_STA_ZPX: Byte = 0x95;
pub const INS_STA_ABS: Byte = 0x8D;
pub const INS_STA_ABSX: Byte = 0x9D;
pub const INS_STA_ABSY: Byte = 0x99;
pub const INS_STA_INDX: Byte = 0x81;
pub const INS_STA_INDY: Byte = 0x91;

// STX
pub const INS_STX_ZP: Byte = 0x86;
pub const INS_STX_ZPY: Byte = 0x96;
pub const INS_STX_ABS: Byte = 0x8E;

// STY
pub const INS_STY_ZP: Byte = 0x84;
pub const INS_STY_ZPX: Byte = 0x94;
pub const INS_STY_ABS: Byte = 0x8C;

pub const INS_TSX: Byte = 0xBA;
pub const INS_TXS: Byte = 0x9A;
pub const INS_PHA: Byte = 0x48;
pub const INS_PLA: Byte = 0x68;
pub const INS_PHP: Byte = 0x08;
pub const INS_PLP: Byte = 0x28;

pub const INS_JMP_ABS: Byte = 0x4C;
pub const INS_JMP_IND: Byte = 0x6C;
pub const INS_JSR: Byte = 0x20;
pub const INS_RTS: Byte = 0x60;

// Logical Ops

// AND
pub const INS_AND_IM: Byte = 0x29;
pub const INS_AND_ZP: Byte = 0x25;
pub const INS_AND_ZPX: Byte = 0x35;
pub const INS_AND_ABS: Byte = 0x2D;
pub const INS_AND_ABSX: Byte = 0x3D;
pub const INS_AND_ABSY: Byte = 0x39;
pub const INS_AND_INDX: Byte = 0x21;
pub const INS_AND_INDY: Byte = 0x31;

// ORA
pub const INS_ORA_IM: Byte = 0x09;
pub const INS_ORA_ZP: Byte = 0x05;
pub const INS_ORA_ZPX: Byte = 0x15;
pub const INS_ORA_ABS: Byte = 0x0D;
pub const INS_ORA_ABSX: Byte = 0x1D;
pub const INS_ORA_ABSY: Byte = 0x19;
pub const INS_ORA_INDX: Byte = 0x01;
pub const INS_ORA_INDY: Byte = 0x11;

// EOR
pub const INS_EOR_IM: Byte = 0x49;
pub const INS_EOR_ZP: Byte = 0x45;
pub const INS_EOR_ZPX: Byte = 0x55;
pub const INS_EOR_ABS: Byte = 0x4D;
pub const INS_EOR_ABSX: Byte = 0x5D;
pub const INS_EOR_ABSY: Byte = 0x59;
pub const INS_EOR_INDX: Byte = 0x41;
pub const INS_EOR_INDY: Byte = 0x51;

// BIT
pub const INS_BIT_ZP: Byte = 0x24;
pub const INS_BIT_ABS: Byte = 0x2C;

// Transfer Registers
pub const INS_TAX: Byte = 0xAA;
pub const INS_TAY: Byte = 0xA8;
pub const INS_TXA: Byte = 0x8A;
pub const INS_TYA: Byte = 0x98;

// Increments, Decrements
pub const INS_INX: Byte = 0xE8;
pub const INS_INY: Byte = 0xC8;
pub const INS_DEY: Byte = 0x88;
pub const INS_DEX: Byte = 0xCA;
pub const INS_DEC_ZP: Byte = 0xC6;
pub const INS_DEC_ZPX: Byte = 0xD6;
pub const INS_DEC_ABS: Byte = 0xCE;
pub const INS_DEC_ABSX: Byte = 0xDE;
pub const INS_INC_ZP: Byte = 0xE6;
pub const INS_INC_ZPX: Byte = 0xF6;
pub const INS_INC_ABS: Byte = 0xEE;
pub const INS_INC_ABSX: Byte = 0xFE;

// Branches
pub const INS_BEQ: Byte = 0xF0;
pub const INS_BNE: Byte = 0xD0;
pub const INS_BCS: Byte = 0xB0;
pub const INS_BCC: Byte = 0x90;
pub const INS_BMI: Byte = 0x30;
pub const INS_BPL: Byte = 0x10;
pub const INS_BVC: Byte = 0x50;
pub const INS_BVS: Byte = 0x70;

// Status Flag Changes
pub const INS_CLC: Byte = 0x18;
pub const INS_SEC: Byte = 0x38;
pub const INS_CLD: Byte = 0xD8;
pub const INS_SED: Byte = 0xF8;
pub const INS_CLI: Byte = 0x58;
pub const INS_SEI: Byte = 0x78;
pub const INS_CLV: Byte = 0xB8;

// Arithmetic
pub const INS_ADC: Byte = 0x69;
pub const INS_ADC_ZP: Byte = 0x65;
pub const INS_ADC_ZPX: Byte = 0x75;
pub const INS_ADC_ABS: Byte = 0x6D;
pub const INS_ADC_ABSX: Byte = 0x7D;
pub const INS_ADC_ABSY: Byte = 0x79;
pub const INS_ADC_INDX: Byte = 0x61;
pub const INS_ADC_INDY: Byte = 0x71;

pub const INS_SBC: Byte = 0xE9;
pub const INS_SBC_ABS: Byte = 0xED;
pub const INS_SBC_ZP: Byte = 0xE5;
pub const INS_SBC_ZPX: Byte = 0xF5;
pub const INS_SBC_ABSX: Byte = 0xFD;
pub const INS_SBC_ABSY: Byte = 0xF9;
pub const INS_SBC_INDX: Byte = 0xE1;
pub const INS_SBC_INDY: Byte = 0xF1;

// Register Comparison
pub const INS_CMP: Byte = 0xC9;
pub const INS_CMP_ZP: Byte = 0xC5;
pub const INS_CMP_ZPX: Byte = 0xD5;
pub const INS_CMP_ABS: Byte = 0xCD;
pub const INS_CMP_ABSX: Byte = 0xDD;
pub const INS_CMP_ABSY: Byte = 0xD9;
pub const INS_CMP_INDX: Byte = 0xC1;
pub const INS_CMP_INDY: Byte = 0xD1;

pub const INS_CPX: Byte = 0xE0;
pub const INS_CPY: Byte = 0xC0;
pub const INS_CPX_ZP: Byte = 0xE4;
pub const INS_CPY_ZP: Byte = 0xC4;
pub const INS_CPX_ABS: Byte = 0xEC;
pub const INS_CPY_ABS: Byte = 0xCC;

// Shifts
pub const INS_ASL: Byte = 0x0A;
pub const INS_ASL_ZP: Byte = 0x06;
pub const INS_ASL_ZPX: Byte = 0x16;
pub const INS_ASL_ABS: Byte = 0x0E;
pub const INS_ASL_ABSX: Byte = 0x1E;

pub const INS_LSR: Byte = 0x4A;
pub const INS_LSR_ZP: Byte = 0x46;
pub const INS_LSR_ZPX: Byte = 0x56;
pub const INS_LSR_ABS: Byte = 0x4E;
pub const INS_LSR_ABSX: Byte = 0x5E;

pub const INS_ROL: Byte = 0x2A;
pub const INS_ROL_ZP: Byte = 0x26;
pub const INS_ROL_ZPX: Byte = 0x36;
pub const INS_ROL_ABS: Byte = 0x2E;
pub const INS_ROL_ABSX: Byte = 0x3E;

pub const INS_ROR: Byte = 0x6A;
pub const INS_ROR_ZP: Byte = 0x66;
pub const INS_ROR_ZPX: Byte = 0x76;
pub const INS_ROR_ABS: Byte = 0x6E;
pub const INS_ROR_ABSX: Byte = 0x7E;

// Misc
pub const INS_NOP: Byte = 0xEA;
pub const INS_BRK: Byte = 0x00;
pub const INS_RTI: Byte = 0x40;
