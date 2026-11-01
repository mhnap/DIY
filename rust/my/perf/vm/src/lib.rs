pub const LOAD: u8 = 0x01;
pub const STORE: u8 = 0x02;
pub const ADD: u8 = 0x03;
pub const SUB: u8 = 0x04;
pub const HALT: u8 = 0xff;

// Stretch goals
pub const ADDI: u8 = 0x05;
pub const SUBI: u8 = 0x06;
pub const JUMP: u8 = 0x07;
pub const BEQZ: u8 = 0x08;

/// Runs the stored program in `memory` to completion, mutating it
/// in place. The result of the program is left at `memory[0]`.
///
/// Memory layout (256 bytes):
///
/// ```text
/// 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f ... ff
/// __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ ... __
/// XX ^==OTHER DATA======^ ^==INSTRUCTIONS=============^
/// ```
///
/// - `memory[0]`     : the output cell (your VM writes its result here)
/// - `memory[1..=7]` : input data and scratch
/// - `memory[8..]`   : program instructions
///
/// Note: all values are 8-bit. Use `wrapping_add` / `wrapping_sub` for
/// arithmetic, including PC arithmetic, since plain `+` / `-` panic on
/// overflow in debug builds.
pub fn compute(memory: &mut [u8; 256]) {
    // Indices match the operand encoding (PC=0x00, r1=0x01, r2=0x02),
    // so once you've decoded an operand byte, `registers[op as usize]` Just Works.
    let registers: [u8; 3] = [8, 0, 0]; // PC, r1, r2

    // Keep looping, like a physical computer's clock.
    loop {
        // Fetch the next opcode.
        let opcode = memory[registers[0] as usize];

        // Decode and execute.
        // For each non-halt instruction, advance the PC past it
        // (3 bytes for the basic ops).
        match opcode {
            HALT => return,
            // LOAD => TODO
            // STORE => TODO
            // ADD => TODO
            // SUB => TODO
            _ => panic!("unimplemented opcode: 0x{opcode:02x}"),
        }
    }
}

#[cfg(test)]
mod tests;
