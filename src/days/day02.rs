use crate::common::Solution;
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

pub fn solve(lines: &[String]) -> Solution {
    let part1: String = solve_part1(lines);
    let part2: String = solve_part2(lines);

    (part1, part2)
}

fn parse_password(s: &String) -> Password {
    let mut p: Password = Password::new();

    let matches = REG.captures(s.as_str()).unwrap();
    p.min = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
    p.max = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
    p.c = matches.get(3).unwrap().as_str().parse::<char>().unwrap();
    p.password = String::from(matches.get(4).unwrap().as_str());

    p
}

fn solve_part1(lines: &[String]) -> String {
    lines
        .iter()
        .map(|x| parse_password(x))
        .filter(|p| {
            let count: u32 = p.password.chars().filter(|x| *x == p.c).count() as u32;
            count >= p.min && count <= p.max
        })
        .count()
        .to_string()
}

fn solve_part2(lines: &[String]) -> String {
    lines
        .iter()
        .map(|x| parse_password(x))
        .filter(|p| {
            let mut result: bool = p.password.chars().nth((p.min - 1) as usize).unwrap() == p.c;
            result ^= p.password.chars().nth((p.max - 1) as usize).unwrap() == p.c;

            result
        })
        .count()
        .to_string()
}
