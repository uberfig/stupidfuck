// use std::vec;
// use regex::Regex;

/// Encapsulates everything required to run a brainfuck program, including its:
/// - RAM
/// - Pointer to memory
/// - Code (instruction data)
/// - Pointer to code (program counter)
#[derive(Debug)]
struct State {
    /// Pointer to memory/RAM (data pointer)
    memptr: usize,
    /// Pointer to code (program counter)
    instptr: usize,
    /// All of RAM
    memory: Vec<u8>,
    /// All code (instruction data)
    inst: Vec<Token>,
    /// Pointer to the last character in the code
    last: usize,
}
impl State {
    fn new() -> Self {
        State { memptr: 0, instptr: 0, memory: Vec::with_capacity(4096), inst: Vec::with_capacity(4096), last: 0 }
    }
}

#[derive(Debug)]
enum Token {
    Right,
    Left,
    Incriment(u8),
    Decriment(u8),
    Open,
    Close,
    Input,
    Output,
}

/// Move data pointer to the right i.e. '>'
fn inc_data(state: &mut State) {
    state.memptr += 1;
    if state.memptr == state.memory.len() {
        state.memory.push(0);
    }
}

/// Move data pointer to the left i.e. '<'
fn dec_data(state: &mut State) {
    state.memptr -= 1;
}

/// Increment value at memory address referenced by the data pointer i.e. '+'
fn incbyte(state: &mut State) {
    state.memory[state.memptr] = state.memory[state.memptr].wrapping_add(1);
}

/// Decrement value at memory address referenced by the data pointer i.e. '-'
fn decbyte(state: &mut State) {
    state.memory[state.memptr] = state.memory[state.memptr].wrapping_sub(1);
}

/// Print out the value at the memory address referenced by the data pointer as an ASCII character to stdout i.e. '.'
fn outbyte(state: &mut State) {
    print!("{}", state.memory[state.memptr] as char);
}

/// Prompt user for a single character via stdin, and once they do that, write that character's ASCII value to the memory address referenced by the data pointer i.e. ','
fn inbyte(state: &mut State) {
    let val = std::io::Read::bytes(std::io::stdin())
        .next()
        .and_then(|result| result.ok())
        .unwrap_or(0);

    state.memory[state.memptr] = val;
}

/// Execute the code inside the following set of square brackets (in code) if the value at the memory address referenced by the data pointer is 0 i.e. '['
/// And keep doing it over and over again until value at the pointed-to memory address is 0.
fn match_forward(state: &mut State) {
    let mut local_level = 1;

    while local_level != 0 {
        state.instptr += 1;
        match state.inst[state.instptr] {
            Token::Open => {
                local_level += 1;
            }
            Token::Close => {
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
        state.instptr -= 1;
        match state.inst[state.instptr] {
            Token::Open => {
                local_level -= 1;
            }
            Token::Close => {
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
    // let re = Regex::new(r"[<>\[\]+\-,.]").unwrap();

    for i in hello {
        // if i.is_ascii() && re.is_match(&(*i as char).to_string()) {
        //     program.inst.push(*i);
        //     curr += 1;
        // }

        match *i {
            b'>' => program.inst.push(Token::Right),
            b'<' => program.inst.push(Token::Left),
            b'+' => program.inst.push(Token::Incriment(1)),
            b'-' => program.inst.push(Token::Decriment(1)),
            b'.' => program.inst.push(Token::Output),
            b',' => program.inst.push(Token::Input),
            b'[' => program.inst.push(Token::Open),
            b']' => program.inst.push(Token::Close),
            _ => {continue;}
        }
        curr += 1;
    }
    program.last = curr;
    program.memory.push(0);

    // dbg!(&program.inst);

    while program.instptr < program.last {
        match program.inst[program.instptr] {
            Token::Right => inc_data(&mut program),
            Token::Left => dec_data(&mut program),
            Token::Incriment(_) => incbyte(&mut program),
            Token::Decriment(_) => decbyte(&mut program),
            Token::Output => outbyte(&mut program),
            Token::Input => inbyte(&mut program),
            Token::Open => {
                if program.memory[program.memptr] == 0 {
                    match_forward(&mut program);
                }
            }
            Token::Close => {
                if program.memory[program.memptr] != 0 {
                    match_rev(&mut program);
                    continue;
                }
            }
        }
        program.instptr += 1;
    }
    println!();
}
