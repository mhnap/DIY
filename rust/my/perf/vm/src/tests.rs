use super::*;

struct VmCase {
    x: u8,
    y: u8,
    out: u8,
}

const fn c(x: u8, y: u8, out: u8) -> VmCase {
    VmCase { x, y, out }
}

// Do nothing, just halt
#[test]
fn halt() {
    run_test("halt", &[c(0, 0, 0)]);
}

// Extra (from HW2_VM.pdf): halt computes nothing, so cell 0 keeps its value.
// run_test resets cell 0, so this test sets up memory itself.
#[test]
fn halt_keeps_output() {
    let mut memory = [0u8; 256];
    memory[0] = 42;
    memory[8] = HALT;

    compute(&mut memory);

    assert_eq!(memory[0], 42, "halt must not change cell 0");
}

// Move a value from input to output
#[test]
fn load_store() {
    run_test(
        "
load r1 1
store r1 0
halt",
        &[c(1, 0, 1), c(255, 0, 255)],
    );
}

// Add two unsigned integers together
#[test]
fn add() {
    run_test(
        "
load r1 1
load r2 2
add r1 r2
store r1 0
halt",
        &[
            c(1, 2, 3),     // 1 + 2 = 3
            c(254, 1, 255), // support max int
            c(255, 1, 0),   // correctly overflow
        ],
    );
}

#[test]
fn subtract() {
    run_test(
        "
load r1 1
load r2 2
sub r1 r2
store r1 0
halt",
        &[
            c(5, 3, 2),
            c(0, 1, 255), // correctly overflow backwards
        ],
    );
}

// Verify that ALU ops use REGISTER values, not memory[operand2].
// The simple add/subtract tests can't catch this bug because r2 is loaded from
// memory[2] and r2's encoding is 0x02, so the two interpretations coincide.
// Here we modify r2 (via sub) before the add, so r2 != memory[2] in general,
// which forces the two interpretations to diverge. Expected output is always y.
#[test]
fn register_arith() {
    run_test(
        "
load r1 1
load r2 2
sub r2 r1
add r1 r2
store r1 0
halt",
        &[
            c(3, 5, 5),     // r2 := 5-3=2; r1 := 3+2=5
            c(10, 20, 20),  // r2 := 10;    r1 := 10+10=20
            c(1, 255, 255), // r2 := 254;   r1 := (1+254) mod 256 = 255
        ],
    );
}

// Extra (from HW2_VM.pdf): registers and memory are separate spaces, r1 is not memory[1].
// Every other test loads r1 from cell 1 and r2 from cell 2, so a VM that keeps
// registers in memory[1..=2] passes them. Crossing the loads makes it diverge:
// such a VM would copy cell 2 over cell 1 and then compute 0.
#[test]
fn registers_not_memory() {
    run_test(
        "
load r1 2
load r2 1
sub r1 r2
store r1 0
halt",
        &[
            c(3, 10, 7),   // r1 := 10; r2 := 3;  10 - 3 = 7
            c(10, 3, 249), // r1 := 3;  r2 := 10; (3 - 10) mod 256 = 249
        ],
    );
}

// Extra (from HW2_VM.pdf): both operands may name the same register
#[test]
fn same_register() {
    run_test(
        "
load r1 1
sub r1 r1
store r1 0
halt",
        &[c(7, 0, 0), c(255, 0, 0)],
    );
}

// Support a basic jump, ie skipping ahead to a particular location
#[test]
#[ignore = "stretch goal"]
fn stretch_jump() {
    run_test(
        "
load r1 1
jump 16
store r1 0
halt",
        &[c(42, 0, 0)],
    );
}

// Support a "branch if equal to zero" with relative offsets
#[test]
#[ignore = "stretch goal"]
fn stretch_beqz() {
    run_test(
        "
load r1 1
load r2 2
beqz r2 3
store r1 0
halt",
        &[
            c(42, 0, 0),  // r2 is zero, so should branch over the store
            c(42, 1, 42), // r2 is nonzero, so should store back 42
        ],
    );
}

// Extra (from HW2_VM.pdf): backward branch with a negative offset.
// beqz is at 0x14 with offset 249 (-7): taken -> 0x14 + 3 - 7 = 0x10 (the store).
#[test]
#[ignore = "stretch goal"]
fn stretch_beqz_negative_offset() {
    run_test(
        "
load r1 1
load r2 2
jump 20
store r1 0
halt
beqz r2 249
halt",
        &[
            c(42, 0, 42), // r2 is zero, so should branch back to the store
            c(42, 1, 0),  // r2 is nonzero, so should fall through to halt
        ],
    );
}

// Support adding immediate values
#[test]
#[ignore = "stretch goal"]
fn stretch_addi() {
    run_test(
        "
load r1 1
addi r1 3
addi r1 5
store r1 0
halt",
        &[
            c(0, 0, 8),   // 0 + 3 + 5 = 8
            c(20, 0, 28), // 20 + 3 + 5 = 28
        ],
    );
}

// Calculate the sum of first n numbers (using subi to decrement loop index)
#[test]
#[ignore = "stretch goal"]
fn stretch_sum_to_n() {
    run_test(
        "
load r1 1
beqz r1 8
add r2 r1
subi r1 1
jump 11
store r2 0
halt",
        &[c(0, 0, 0), c(1, 0, 1), c(5, 0, 15), c(10, 0, 55)],
    );
}

// Given some assembly code and test cases, construct a program
// according to the required memory structure, and run in each
// case through the virtual machine
fn run_test(asm: &str, cases: &[VmCase]) {
    // assemble code and load into memory
    let mut memory = [0u8; 256];
    let code = assemble(asm);
    memory[8..8 + code.len()].copy_from_slice(&code);
    // for each case, set inputs and run vm
    for case in cases {
        // Reset the input/output cells so each case starts from a known state.
        // Without this, an output left over from the previous case could mask
        // a VM that forgets to write memory[0].
        memory[0] = 0;
        memory[1] = case.x;
        memory[2] = case.y;

        compute(&mut memory);

        let actual = memory[0];
        assert_eq!(
            actual, case.out,
            "Expected f({}, {}) to be {}, not {}",
            case.x, case.y, case.out, actual
        );
    }
}

fn reg(s: &str) -> u8 {
    match s {
        "r1" => 0x01,
        "r2" => 0x02,
        _ => panic!("Invalid register: {s}"),
    }
}

fn mem(s: &str) -> u8 {
    s.parse::<i64>().unwrap() as u8
}

fn imm(s: &str) -> u8 {
    // for now, immediate values and memory addresses are both just ints
    mem(s)
}

// Assemble the given assembly code to machine code
fn assemble(asm: &str) -> Vec<u8> {
    let mut mc = Vec::new();
    for line in asm.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "load" => mc.extend([LOAD, reg(parts[1]), mem(parts[2])]),
            "store" => mc.extend([STORE, reg(parts[1]), mem(parts[2])]),
            "add" => mc.extend([ADD, reg(parts[1]), reg(parts[2])]),
            "sub" => mc.extend([SUB, reg(parts[1]), reg(parts[2])]),
            "addi" => mc.extend([ADDI, reg(parts[1]), imm(parts[2])]),
            "subi" => mc.extend([SUBI, reg(parts[1]), imm(parts[2])]),
            "jump" => mc.extend([JUMP, imm(parts[1])]),
            "beqz" => mc.extend([BEQZ, reg(parts[1]), imm(parts[2])]),
            "halt" => mc.push(HALT),
            op => panic!("Invalid operation: {op}"),
        }
    }
    mc
}
