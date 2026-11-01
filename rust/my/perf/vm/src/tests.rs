use super::compute;

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

// Support a basic jump, ie skipping ahead to a particular location
#[test]
fn stretch_jump() {
    run_stretch_test(
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
fn stretch_beqz() {
    run_stretch_test(
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

// Support adding immediate values
#[test]
fn stretch_addi() {
    run_stretch_test(
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
fn stretch_sum_to_n() {
    run_stretch_test(
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

fn run_stretch_test(asm: &str, cases: &[VmCase]) {
    if std::env::var("STRETCH").as_deref() != Ok("true") {
        println!("Skipping stretch goal tests. Run `STRETCH=true cargo test` to include them.");
        return;
    }
    run_test(asm, cases);
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
            "load" => mc.extend([0x01, reg(parts[1]), mem(parts[2])]),
            "store" => mc.extend([0x02, reg(parts[1]), mem(parts[2])]),
            "add" => mc.extend([0x03, reg(parts[1]), reg(parts[2])]),
            "sub" => mc.extend([0x04, reg(parts[1]), reg(parts[2])]),
            "addi" => mc.extend([0x05, reg(parts[1]), imm(parts[2])]),
            "subi" => mc.extend([0x06, reg(parts[1]), imm(parts[2])]),
            "jump" => mc.extend([0x07, imm(parts[1])]),
            "beqz" => mc.extend([0x08, reg(parts[1]), imm(parts[2])]),
            "halt" => mc.push(0xff),
            op => panic!("Invalid operation: {op}"),
        }
    }
    mc
}
