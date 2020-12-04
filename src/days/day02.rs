use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(\d+)-(\d+)\s+(\w+):\s+(\w+)").unwrap();
}

#[derive(Clone, Debug)]
struct Password {
    min: u32,
    max: u32,
    c: char,
    password: String,
}

impl Password {
    fn new() -> Password {
        Password {
            min: 0,
            max: 0,
            c: 'a',
            password: String::new(),
        }
    }
}

pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn parse_password(s: &str) -> Password {
    let mut p: Password = Password::new();

    let matches = REG.captures(s).unwrap();
    p.min = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
    p.max = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
    p.c = matches.get(3).unwrap().as_str().parse::<char>().unwrap();
    p.password = String::from(matches.get(4).unwrap().as_str());

    p
}

fn solve_part1(input: &str) {
    let result = input
        .split("\n")
        .map(|x| parse_password(x))
        .filter(|p| {
            let count: u32 = p.password.chars().filter(|x| *x == p.c).count() as u32;
            count >= p.min && count <= p.max
        })
        .count();

    println!("part 1: {}", result);
}

fn solve_part2(input: &str) {
    let result = input
        .split("\n")
        .map(|x| parse_password(x))
        .filter(|p| {
            let mut result: bool = p.password.chars().nth((p.min - 1) as usize).unwrap() == p.c;
            result ^= p.password.chars().nth((p.max - 1) as usize).unwrap() == p.c;

            result
        })
        .count();

    println!("part 2: {}", result);
}
