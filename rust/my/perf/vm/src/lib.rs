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
    // so once you've decoded an operand byte, `regs[op as usize]` Just Works.
    let mut regs: [u8; 3] = [8, 0, 0]; // PC, r1, r2

    // Keep looping, like a physical computer's clock.
    loop {
        // Fetch the next opcode.
        let opcode = memory[regs[0] as usize];
        regs[0] += 1;

        // Decode and execute.
        // For each non-halt instruction, advance the PC past it
        // (3 bytes for the basic ops).
        match opcode {
            HALT => return,
            LOAD => {
                // Read register index from memory by PC.
                let reg_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read memory address from memory by PC.
                let mem_addr = memory[regs[0] as usize];
                regs[0] += 1;

                // Read memory value by memory address.
                let mem_val = memory[mem_addr as usize];

                // Write memory value to register by index.
                regs[reg_idx as usize] = mem_val;
            }
            STORE => {
                // Read register index from memory by PC.
                let reg_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read memory address from memory by PC.
                let mem_addr = memory[regs[0] as usize];
                regs[0] += 1;

                // Read register value by register index.
                let reg_val = regs[reg_idx as usize];

                // Write register value to memory by memory address.
                memory[mem_addr as usize] = reg_val;
            }
            ADD => {
                // Read destination register index from memory by PC.
                let reg_dst_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read source register index from memory by PC.
                let reg_src_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read destination register value by index.
                let reg_dst_val = regs[reg_dst_idx as usize];

                // Read source register value by index.
                let reg_src_val = regs[reg_src_idx as usize];

                // Add source to destination register.
                regs[reg_dst_idx as usize] = reg_dst_val.wrapping_add(reg_src_val);
            }
            SUB => {
                // Read destination register index from memory by PC.
                let reg_dst_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read source register index from memory by PC.
                let reg_src_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read destination register value by index.
                let reg_dst_val = regs[reg_dst_idx as usize];

                // Read source register value by index.
                let reg_src_val = regs[reg_src_idx as usize];

                // Subtract source from destination register.
                regs[reg_dst_idx as usize] = reg_dst_val.wrapping_sub(reg_src_val);
            }
            ADDI => {
                // Read destination register index from memory by PC.
                let reg_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read immediate value from memory by PC.
                let imm_val = memory[regs[0] as usize];
                regs[0] += 1;

                // Write value to register by index.
                regs[reg_idx as usize] = regs[reg_idx as usize].wrapping_add(imm_val);
            }
            SUBI => {
                // Read destination register index from memory by PC.
                let reg_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read immediate value from memory by PC.
                let imm_val = memory[regs[0] as usize];
                regs[0] += 1;

                // Write value to register by index.
                regs[reg_idx as usize] = regs[reg_idx as usize].wrapping_sub(imm_val);
            }
            JUMP => {
                // Read jump address.
                let jump = memory[regs[0] as usize];

                // Jump by changing PC.
                regs[0] = jump;
            }
            BEQZ => {
                // Read register index from memory by PC.
                let reg_idx = memory[regs[0] as usize];
                regs[0] += 1;

                // Read offset by PC.
                let offset = memory[regs[0] as usize];
                regs[0] += 1;

                // Convert to signed.
                let offset = offset as i8;

                if regs[reg_idx as usize] == 0 {
                    regs[0] = regs[0].wrapping_add_signed(offset);
                } else {
                    // Fall through.
                }
            }
            _ => panic!("unimplemented opcode: 0x{opcode:02x}"),
        }
    }
}

#[cfg(test)]
mod tests;
