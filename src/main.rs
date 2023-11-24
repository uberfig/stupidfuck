// use std::vec;
// use regex::Regex;

use std::usize;

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

#[derive(Debug, Clone, Copy)]
enum Token {
    Right(usize),
    Left(usize),
    Incriment(u8),
    Decriment(u8),
    Open(usize),
    Close(usize),
    Input,
    Output,
}

/// Move data pointer to the right i.e. '>'
fn inc_data(state: &mut State, amount: usize) {
    state.memptr += amount;
    if state.memptr >= state.memory.len() {
        for _i in 0..=state.memptr-state.memory.len() {
            state.memory.push(0);
        }
    }
}

/// Move data pointer to the left i.e. '<'
fn dec_data(state: &mut State, amount: usize) {
    state.memptr -= amount;
}

/// Increment value at memory address referenced by the data pointer i.e. '+'
fn incbyte(state: &mut State, amount: u8) {
    state.memory[state.memptr] = state.memory[state.memptr].wrapping_add(amount);
}

/// Decrement value at memory address referenced by the data pointer i.e. '-'
fn decbyte(state: &mut State, amount: u8) {
    state.memory[state.memptr] = state.memory[state.memptr].wrapping_sub(amount);
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

/// Find the position of the closing ]
fn forward_ofset(state: &mut State, pos: usize) -> usize {
    let mut local_level = 1;
    let mut pos: usize = pos;
    while local_level != 0 {
        pos += 1;
        match state.inst[pos] {
            Token::Open(_) => {
                local_level += 1;
            }
            Token::Close(_) => {
                local_level -= 1;
            }
            _ => {}
        }
    }
    pos
}

/// Execute the code inside the following set of square brackets (in code) if the value at the memory address referenced by the data pointer is 0 i.e. '['
/// And keep doing it over and over again until value at the pointed-to memory address is 0.
fn jump_forward(state: &mut State, pos: usize) {
    state.instptr = pos;
}

///calculate the matching [ to a ]
fn rev_ofset(state: &mut State, pos: usize) -> usize {
    let mut pos = pos;
    let mut local_level = 1;
    while local_level != 0 {
        pos -= 1;
        match state.inst[pos] {
            Token::Open(_) => {
                local_level -= 1;
            }
            Token::Close(_) => {
                local_level += 1;
            }
            _ => {}
        }
    }
    pos
}

/// Signify the end of a repeated code section i.e. ']'
fn jump_rev(state: &mut State, pos: usize) {
    state.instptr = pos;
}

fn main() {
    let hello = include_str!("../hello.bf").as_bytes();
    let mut program = State::new();
    let mut curr: usize = 0;

    for i in hello {
        match *i {
            b'>' => program.inst.push(Token::Right(1)),
            b'<' => program.inst.push(Token::Left(1)),
            b'+' => program.inst.push(Token::Incriment(1)),
            b'-' => program.inst.push(Token::Decriment(1)),
            b'.' => program.inst.push(Token::Output),
            b',' => program.inst.push(Token::Input),
            b'[' => program.inst.push(Token::Open(1)),
            b']' => program.inst.push(Token::Close(1)),
            _ => {continue;}
        }
        curr += 1;
    }
    program.last = curr;
    program.memory.push(0);

    let mut new_inst: Vec<Token> = Vec::with_capacity(4096);
    
    for i in 0..program.last {
        match program.inst[i] {
            Token::Right(_) => {
                if new_inst.len() != 0 {
                    let val = new_inst[new_inst.len()-1];
                    match val {
                        Token::Right(b) => {
                            let pos = new_inst.len()-1;
                            new_inst[pos] = Token::Right(b+1);
                        },
                        _ => new_inst.push(Token::Right(1)),
                    }
                } else {
                    new_inst.push(Token::Right(1));
                }
            },
            Token::Left(_) => {
                if new_inst.len() != 0 {
                    let val = new_inst[new_inst.len()-1];
                    match val {
                        Token::Left(b) => {
                            let pos = new_inst.len()-1;
                            new_inst[pos] = Token::Left(b+1);
                        },
                        _ => new_inst.push(Token::Left(1)),
                    }
                } else {
                    new_inst.push(Token::Left(1));
                }
            },
            Token::Incriment(_) => {
                if new_inst.len() != 0 {
                    let val = new_inst[new_inst.len()-1];
                    match val {
                        Token::Incriment(b) => {
                            let pos = new_inst.len()-1;
                            new_inst[pos] = Token::Incriment(b.wrapping_add(1));
                        },
                        _ => new_inst.push(Token::Incriment(1)),
                    }
                } else {
                    new_inst.push(Token::Incriment(1));
                }
            },
            Token::Decriment(_) => {
                if new_inst.len() != 0 {
                    let val = new_inst[new_inst.len()-1];
                    match val {
                        Token::Decriment(b) => {
                            let pos = new_inst.len()-1;
                            new_inst[pos] = Token::Decriment(b.wrapping_add(1));
                        },
                        _ => new_inst.push(Token::Decriment(1)),
                    }
                } else {
                    new_inst.push(Token::Decriment(1));
                }
            },
            _ => new_inst.push(program.inst[i]),
        }
    }

    program.inst = new_inst;

    for i in 0..program.inst.len() {
        match program.inst[i] {
            Token::Open(_) => {
                let pos = forward_ofset(&mut program, i);
                program.inst[i] = Token::Open(pos);
            },
            Token::Close(_) => {
                let pos = rev_ofset(&mut program, i);
                program.inst[i] = Token::Close(pos);
            },
            _ => {},
        }
    }

    while program.instptr < program.inst.len() {
        match program.inst[program.instptr] {
            Token::Right(a) => inc_data(&mut program, a),
            Token::Left(a) => dec_data(&mut program, a),
            Token::Incriment(a) => incbyte(&mut program, a),
            Token::Decriment(a) => decbyte(&mut program, a),
            Token::Output => outbyte(&mut program),
            Token::Input => inbyte(&mut program),
            Token::Open(a) => {
                if program.memory[program.memptr] == 0 {
                    jump_forward(&mut program, a);
                }
            }
            Token::Close(a) => {
                if program.memory[program.memptr] != 0 {
                    jump_rev(&mut program, a);
                    continue;
                }
            }
        }
        program.instptr += 1;
    }
    println!();
}
