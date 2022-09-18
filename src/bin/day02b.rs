pub enum Operation {
    Add,
    Multiply,
    Terminate,
}

pub struct ParseOpcodeError;

impl Operation {
    fn from_opcode(opcode: i32) -> Result<Operation, ParseOpcodeError> {
        match opcode {
            1 => Ok(Operation::Add),
            2 => Ok(Operation::Multiply),
            99 => Ok(Operation::Terminate),
            _ => Err(ParseOpcodeError),
        }
    }
}

fn parse_code(str: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    return str.trim().split(',').map(|x| x.parse::<i32>()).collect();
}

#[derive(Debug)]
pub struct ExecError;

fn exec_op(op: Operation, code: &mut [i32], ptr: usize) -> Result<(), ExecError> {
    let fst_pos = code[ptr + 1] as usize;
    let snd_pos = code[ptr + 2] as usize;
    let dest = code[ptr + 3] as usize;
    match op {
        Operation::Add => {
            code[dest] = code[fst_pos] + code[snd_pos];
            Ok(())
        }
        Operation::Multiply => {
            code[dest] = code[fst_pos] * code[snd_pos];
            Ok(())
        }
        _ => Err(ExecError),
    }
}

fn exec_code(code: &mut Vec<i32>, ptr: usize) -> Result<(), ExecError> {
    Operation::from_opcode(code[ptr])
        .map_err(|_| ExecError {})
        .and_then(|op| match op {
            Operation::Terminate => Ok(()),
            _ => exec_op(op, code, ptr).and_then(|_| exec_code(code, ptr + 4)),
        })
}

fn main() {
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line);
    let code = parse_code(&line).unwrap();
    let mut found = false;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = code.clone();
            memory[1] = noun;
            memory[2] = verb;

            exec_code(&mut memory, 0).unwrap();
            if memory[0] == 19690720 {
                found = true;
                println!("{}", 100 * noun + verb);
                break;
            }
        }

        if found {
            break;
        }
    }
}
