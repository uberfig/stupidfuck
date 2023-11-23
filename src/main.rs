#[derive(Debug)]
struct State {
    data: usize,
    ir: usize,
    memory: [u8; 512],
    inst: [u8; 512],
    last: usize,
}
fn inc_data(state: &mut State) {
    state.data += 1;
}

fn dec_data(state: &mut State) {
    state.data -= 1;
}

fn incbyte(state: &mut State) {
    state.memory[state.data] = state.memory[state.data].wrapping_add(1);
}

fn decbyte(state: &mut State) {
    state.memory[state.data] = state.memory[state.data].wrapping_sub(1);
}

fn outbyte(state: &mut State) {
    print!("{}", state.memory[state.data] as char);
}

fn inbyte(state: &mut State) {
    let val = std::io::Read::bytes(std::io::stdin())
        .next()
        .and_then(|result| result.ok())
        .unwrap_or(0);

    state.memory[state.data] = val;
}

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
        // I Copied the wrong line of code and
        // didn't realize i t
        //local_level -= 1;
    }
}

fn main() {
    let hello = include_str!("../hello.bf").as_bytes();
    let mut program = State {
        data: 0,
        ir: 0,
        memory: [0; 512],
        inst: [0; 512],
        last: 0,
    };
    let mut curr: usize = 0;
    for i in hello {
        match i {
            b'>' => program.inst[curr] = b'>',
            b'<' => program.inst[curr] = b'<',
            b'+' => program.inst[curr] = b'+',
            b'-' => program.inst[curr] = b'-',
            b'.' => program.inst[curr] = b'.',
            b',' => program.inst[curr] = b',',
            b'[' => program.inst[curr] = b'[',
            b']' => program.inst[curr] = b']',
            _ => {
                continue;
            }
        }
        curr += 1;
    }
    program.last = curr;

    // let mut level = 0;

    while program.ir < program.last {
        // let i = program.inst[program.ir];
        // let bruh = i as char;
        // let data = program.memory[program.data];

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
