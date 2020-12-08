use std::collections::HashSet;

pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let result: usize = input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    println!("part 1: {}", result);
}

fn solve_part2(input: &str) {
    let result: usize = input
        .trim()
        .split("\n\n")
        .map(|group| {
            let char_set = group
                .lines()
                .next()
                .unwrap()
                .chars()
                .collect::<HashSet<_>>();
            group
                .lines()
                .fold(char_set, |char_set, line| {
                    &char_set & &line.chars().collect::<HashSet<_>>()
                })
                .len()
        })
        .sum::<usize>();

    println!("part 2: {}", result);
}
