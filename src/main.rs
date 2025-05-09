use m6502::{Cpu, Memory, Word};
use std::env;
use std::fs;

fn main() {
    // Create memory with all zeros.
    let mut memory = Memory { data: [0; 1024 * 64] };

    // Set the reset vector so that the CPU starts execution at 0x8000.
    memory.data[0xFFFC] = 0x00; // low byte
    memory.data[0xFFFD] = 0x80; // high byte

    // Determine start address.
    let start: Word = 0x8000;

    // Check if a program file was passed as an argument.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Read the program file as binary.
        let filename = &args[1];
        match fs::read(filename) {
            Ok(program_bytes) => {
                if start as usize + program_bytes.len() > memory.data.len() {
                    eprintln!("Error: Program too large to fit in memory");
                    return;
                }
                memory.data[start as usize..start as usize + program_bytes.len()]
                    .copy_from_slice(&program_bytes);
                println!("Program loaded from '{}'", filename);
            }
            Err(e) => {
                eprintln!("Error reading '{}': {}", filename, e);
                return;
            }
        }
    } else {
        // Load default program (INS_LDA_IM 0x42, INS_NOP, INS_BRK).
        memory.data[start as usize]     = 0xA9; // INS_LDA_IM
        memory.data[start as usize + 1] = 0x42; // Value to load into A
        memory.data[start as usize + 2] = 0xEA; // INS_NOP
        memory.data[start as usize + 3] = 0x00; // INS_BRK
        println!("Default test program loaded");
    }

    // Create a CPU instance and reset it.
    let mut cpu = Cpu::new();
    cpu.reset(&mut memory);

    // Execute the program for a limited number of cycles.
    let cycles = 20;
    let cycles_consumed = cpu.execute(cycles, &mut memory);

    // Print CPU state after execution.
    println!("After execution:");
    println!("Accumulator (A): {:#X}", cpu.reg_a);
    println!("Register X:      {:#X}", cpu.reg_x);
    println!("Register Y:      {:#X}", cpu.reg_y);
    println!("Program Counter: {:#X}", cpu.pc);
    println!("Status Flags:");
    println!("  Carry:              {}", cpu.status.carry);
    println!("  Zero:               {}", cpu.status.zero);
    println!("  Interrupt Disable:  {}", cpu.status.interrupt_disable);
    println!("  Decimal Mode:       {}", cpu.status.decimal_mode);
    println!("  Break:              {}", cpu.status.break_command);
    println!("  Overflow:           {}", cpu.status.overflow);
    println!("  Negative:           {}", cpu.status.negative);
    println!("Cycles consumed: {}", cycles_consumed);
}

