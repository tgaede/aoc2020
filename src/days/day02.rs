use crate::common::Solution;

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
    let mut line_iter = s.split_whitespace();

    let minmax_string = line_iter.next().unwrap().to_string();
    let mut minmax_iter = minmax_string.split('-');
    p.min = minmax_iter
        .next()
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();
    p.max = minmax_iter
        .next()
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();
    p.c = line_iter
        .next()
        .unwrap()
        .to_string()
        .chars()
        .next()
        .unwrap();
    p.password = line_iter.next().unwrap().to_string();

    p
}

fn is_password_valid_part1(p: &Password) -> bool {
    let count: u32 = p.password.chars().filter(|x| *x == p.c).count() as u32;

    count >= p.min && count <= p.max
}

fn solve_part1(lines: &[String]) -> String {
    return lines
        .iter()
        .map(|x| parse_password(x))
        .filter(|x| is_password_valid_part1(x))
        .count()
        .to_string();
}

fn is_password_valid_part2(p: &Password) -> bool {
    let mut result: bool = p.password.chars().nth((p.min - 1) as usize).unwrap() == p.c;
    result ^= p.password.chars().nth((p.max - 1) as usize).unwrap() == p.c;

    result
}

fn solve_part2(lines: &[String]) -> String {
    return lines
        .iter()
        .map(|x| parse_password(x))
        .filter(|x| is_password_valid_part2(x))
        .count()
        .to_string();
}
