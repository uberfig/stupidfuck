use regex::Regex;

/// Encapsulates everything required to run a brainfuck program, including its:
/// - RAM
/// - Pointer to memory
/// - Code (instruction data)
/// - Pointer to code (program counter)
#[derive(Debug)]
struct State {
    /// Pointer to memory/RAM (data pointer)
    data: usize,
    /// Pointer to code (program counter)
    ir: usize,
    /// All of RAM
    memory: [u8; 4096],
    /// All code (instruction data)
    inst: [u8; 4096],
    /// Pointer to the last character in the code
    last: usize,
}
impl State {
    fn new() -> Self {
        State { data: 0, ir: 0, memory: [0; 4096], inst: [0; 4096], last: 0 }
    }
}

/// Move data pointer to the right i.e. '>'
fn inc_data(state: &mut State) {
    state.data += 1;
}

/// Move data pointer to the left i.e. '<'
fn dec_data(state: &mut State) {
    state.data -= 1;
}

/// Increment value at memory address referenced by the data pointer i.e. '+'
fn incbyte(state: &mut State) {
    state.memory[state.data] = state.memory[state.data].wrapping_add(1);
}

/// Decrement value at memory address referenced by the data pointer i.e. '-'
fn decbyte(state: &mut State) {
    state.memory[state.data] = state.memory[state.data].wrapping_sub(1);
}

/// Print out the value at the memory address referenced by the data pointer as an ASCII character to stdout i.e. '.'
fn outbyte(state: &mut State) {
    print!("{}", state.memory[state.data] as char);
}

/// Prompt user for a single character via stdin, and once they do that, write that character's ASCII value to the memory address referenced by the data pointer i.e. ','
fn inbyte(state: &mut State) {
    let val = std::io::Read::bytes(std::io::stdin())
        .next()
        .and_then(|result| result.ok())
        .unwrap_or(0);

    state.memory[state.data] = val;
}

/// Execute the code inside the following set of square brackets (in code) if the value at the memory address referenced by the data pointer is 0 i.e. '['
/// And keep doing it over and over again until value at the pointed-to memory address is 0.
fn match_forward(state: &mut State) {
    let mut local_level = 1;

    while local_level != 0 {
        state.ir += 1;
        match state.inst[state.ir] {
            b'[' => {
                local_level += 1;
            }
            b']' => {
                local_level -= 1;
            }
            _ => {}
        }
    }
}

/// Signify the end of a repeated code section i.e. ']'
fn match_rev(state: &mut State) {
    let mut local_level = 1;

    while local_level != 0 {
        state.ir -= 1;
        match state.inst[state.ir] {
            b'[' => {
                local_level -= 1;
            }
            b']' => {
                local_level += 1;
            }
            _ => {}
        }
    }
}

fn main() {
    let hello = include_str!("../hello.bf").as_bytes();
    let mut program = State::new();
    let mut curr: usize = 0;
    let re = Regex::new(r"[<>\[\]+\-,.]").unwrap();

    for i in hello {
        if i.is_ascii() && re.is_match(&(*i as char).to_string()) {
            program.inst[curr] = *i;
        }
        curr += 1;
    }
    program.last = curr;

    while program.ir < program.last {

        match program.inst[program.ir] {
            b'>' => inc_data(&mut program),
            b'<' => dec_data(&mut program),
            b'+' => incbyte(&mut program),
            b'-' => decbyte(&mut program),
            b'.' => outbyte(&mut program),
            b',' => inbyte(&mut program),
            b'[' => {
                if program.memory[program.data] == 0 {
                    match_forward(&mut program);
                }
            }
            b']' => {
                if program.memory[program.data] != 0 {
                    match_rev(&mut program);
                    continue;
                }
            }
            _ => {}
        }
        program.ir += 1;
    }
    println!();
}
