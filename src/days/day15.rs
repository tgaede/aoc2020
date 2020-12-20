use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    get_numbers(&input, 2021)
}

fn get_numbers(input: &str, until: u32) -> usize {
    let mut map: HashMap<u32, u32>;
    map = input
        .trim()
        .split(",")
        .enumerate()
        .map(|(i, x)| (x.parse::<u32>().unwrap(), i as u32 + 1))
        .collect();

    let mut index: u32 = (map.len() + 1) as u32;
    let mut last_number: u32 = input
        .trim()
        .rsplit(",")
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut last_index: u32 = 0;
    let mut spoken_before: bool = false;

    while index < until {
        let new_number: u32;
        if !spoken_before {
            new_number = 0;
        } else {
            new_number = (index - 1) - last_index;
        }
        if map.contains_key(&new_number) {
            spoken_before = true;
            last_index = map[&new_number];
        } else {
            spoken_before = false;
        }
        map.insert(new_number, index);
        last_number = new_number;
        index += 1;
    }

    last_number as usize
}

fn solve_part2(input: &str) -> usize {
    get_numbers(&input, 30000001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        assert_eq!(solve_part1("2,1,3"), 10);
        assert_eq!(solve_part1("1,3,2"), 1);
        assert_eq!(solve_part1("2,3,1"), 78);
        assert_eq!(solve_part1("3,2,1"), 438);
        assert_eq!(solve_part1("3,1,2"), 1836);
        assert_eq!(solve_part1("0,3,6"), 436);
    }

    // these tests take too long on non-release builds
    // #[test]
    // fn test_day15_part2() {
    //     assert_eq!(solve_part2("0,3,6"), 175594);
    //     assert_eq!(solve_part2("1,3,2"), 2578);
    //     assert_eq!(solve_part2("2,1,3"), 3544142);
    //     assert_eq!(solve_part2("1,2,3"), 261214);
    //     assert_eq!(solve_part2("2,3,1"), 6895259);
    //     assert_eq!(solve_part2("3,2,1"), 18);
    //     assert_eq!(solve_part2("3,1,2"), 362);
    // }
}
