const OP_MULT: char = '*';
const OP_ADD: char = '+';
const OP_NONE: char = ' ';

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> i64 {
    input
        .trim()
        .split("\n")
        .map(|line| calculate_part1(line))
        .sum()
}

fn calculate_part1(input: &str) -> i64 {
    let mut index: usize = 0;
    let mut char_iter = (*input).chars();
    let mut op: char = OP_NONE;
    let mut operations: Vec<(char, i64)> = Vec::new();

    while let Some(c) = char_iter.next() {
        match c {
            '(' => {
                let end_index: usize = find_matching_paren_index(input, index);
                operations.push((
                    op,
                    calculate_part1(input.get(index + 1..end_index).unwrap()),
                ));
                char_iter.nth(end_index - index - 1);
                index = end_index;
            }
            '*' => {
                op = OP_MULT;
            }
            '+' => {
                op = OP_ADD;
            }
            ' ' => {}
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                operations.push((op, c.to_digit(10).unwrap() as i64));
            }
            _ => unreachable!(),
        }

        index += 1;
    }

    let mut i: usize = 0;
    while i < operations.len() {
        if operations[i].0 == OP_ADD {
            operations[i - 1] = (operations[i - 1].0, operations[i].1 + operations[i - 1].1);
            operations.remove(i);
            i -= 1;
        } else if operations[i].0 == OP_MULT {
            operations[i - 1] = (operations[i - 1].0, operations[i].1 * operations[i - 1].1);
            operations.remove(i);
            i -= 1;
        }
        i += 1;
    }

    operations[0].1
}

fn calculate_part2(input: &str) -> i64 {
    let mut index: usize = 0;
    let mut char_iter = (*input).chars();
    let mut op: char = OP_NONE;
    let mut operations: Vec<(char, i64)> = Vec::new();

    while let Some(c) = char_iter.next() {
        match c {
            '(' => {
                let end_index: usize = find_matching_paren_index(input, index);
                operations.push((
                    op,
                    calculate_part2(input.get(index + 1..end_index).unwrap()),
                ));
                char_iter.nth(end_index - index - 1);
                index = end_index;
            }
            '*' => {
                op = OP_MULT;
            }
            '+' => {
                op = OP_ADD;
            }
            ' ' => {}
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                operations.push((op, c.to_digit(10).unwrap() as i64));
            }
            _ => unreachable!(),
        }

        index += 1;
    }

    let mut i: usize = 0;
    while i < operations.len() {
        if operations[i].0 == OP_ADD {
            operations[i - 1] = (operations[i - 1].0, operations[i].1 + operations[i - 1].1);
            operations.remove(i);
            i -= 1;
        }
        i += 1;
    }
    i = 0;
    while i < operations.len() {
        if operations[i].0 == OP_MULT {
            operations[i - 1] = (operations[i - 1].0, operations[i].1 * operations[i - 1].1);
            operations.remove(i);
            i -= 1;
        }
        i += 1;
    }

    operations[0].1
}

fn find_matching_paren_index(input: &str, start: usize) -> usize {
    let mut paren_count: u32 = 0;
    for (i, c) in (*input).chars().enumerate().skip(start) {
        match c {
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            _ => {}
        };

        if paren_count == 0 {
            return i;
        }
    }

    0
}

fn solve_part2(input: &str) -> i64 {
    input
        .trim()
        .split("\n")
        .map(|line| calculate_part2(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_part1() {
        assert_eq!(solve_part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(solve_part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            solve_part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            solve_part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_day18_part2() {
        assert_eq!(solve_part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve_part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(solve_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            solve_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            solve_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
