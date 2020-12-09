use std::collections::HashSet;

const OP_ACC: &str = "acc";
const OP_JMP: &str = "jmp";
const OP_NOP: &str = "nop";

#[derive(Clone, Debug)]
struct Instruction {
    operation: &'static str,
    argument: i64,
}

impl Instruction {
    fn new(op: &'static str, arg: i64) -> Instruction {
        Instruction {
            operation: op,
            argument: arg,
        }
    }
}

#[derive(Clone, Debug)]
struct Computer {
    memory: Vec<Instruction>,
    ip: usize,
    accumulator: i64,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            memory: Vec::new(),
            ip: 0,
            accumulator: 0,
        }
    }
}

fn parse_computer(input: &str) -> Computer {
    let mut c: Computer = Computer::new();

    input.split("\n").for_each(|line| {
        let mut line_parts = line.split_ascii_whitespace();
        let instruction = line_parts.next().unwrap();
        let arg = line_parts.next().unwrap();
        match instruction {
            OP_ACC => c
                .memory
                .push(Instruction::new(OP_ACC, arg.parse::<i64>().unwrap())),
            OP_JMP => c
                .memory
                .push(Instruction::new(OP_JMP, arg.parse::<i64>().unwrap())),
            OP_NOP => c
                .memory
                .push(Instruction::new(OP_NOP, arg.parse::<i64>().unwrap())),
            _ => unreachable!(),
        }
    });

    c
}

fn step_computer(computer: &mut Computer) {
    match computer.memory[computer.ip].operation {
        OP_ACC => {
            computer.accumulator += computer.memory[computer.ip].argument;
            computer.ip += 1;
        }
        OP_JMP => {
            computer.ip = (computer.ip as i64 + computer.memory[computer.ip].argument) as usize;
        }
        OP_NOP => {
            computer.ip += 1;
        }
        _ => unreachable!(),
    }
}

fn run_computer(computer: &mut Computer) -> bool {
    let mut visited: HashSet<usize> = HashSet::new();

    while !visited.contains(&computer.ip) && computer.ip < computer.memory.len() {
        visited.insert(computer.ip);
        step_computer(computer);
    }

    computer.ip == computer.memory.len()
}

pub fn solve(input: &str) {
    // let test = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
    // solve_part1(test);

    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let mut computer = parse_computer(input);
    run_computer(&mut computer);
    println!("part 1: {:#?}", computer);
}

fn solve_part2(input: &str) {
    let computer = parse_computer(input);

    for i in 0..computer.memory.len() {
        if computer.memory[i].operation == OP_NOP || computer.memory[i].operation == OP_JMP {
            let mut try_computer = computer.clone();
            if try_computer.memory[i].operation == OP_NOP {
                try_computer.memory[i].operation = OP_JMP;
            } else {
                try_computer.memory[i].operation = OP_NOP;
            }
            if run_computer(&mut try_computer) {
                println!(
                    "found a computer that works: {}, acc: {}",
                    i, try_computer.accumulator
                );
                break;
            }
        }
    }

    println!("part 2: {}", "");
}
