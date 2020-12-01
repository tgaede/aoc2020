use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let part1: String = solve_part1(lines);
    let part2: String = solve_part2(lines);

    (part1, part2)
}

fn solve_part1(lines: &[String]) -> String {
    let mut numbers: Vec<u32> = lines.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.sort();

    for x in 0..numbers.len() {
        for y in (x + 1)..numbers.len() {
            if (numbers[x] + numbers[y]) == 2020 {
                return (numbers[x] * numbers[y]).to_string();
            } else if (numbers[x] + numbers[y]) > 2020 {
                break;
            }
        }
    }

    unreachable!();
}

fn solve_part2(lines: &[String]) -> String {
    let mut numbers: Vec<u32> = lines.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.sort();

    for x in 0..numbers.len() {
        for y in (x + 1)..numbers.len() {
            for z in (y + 1)..numbers.len() {
                if (numbers[x] + numbers[y] + numbers[z]) == 2020 {
                    return (numbers[x] * numbers[y] * numbers[z]).to_string();
                } else if (numbers[x] + numbers[y] + numbers[z]) > 2020 {
                    break;
                }
            }
        }
    }

    unreachable!();
}
