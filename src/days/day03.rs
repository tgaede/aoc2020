use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let part1: String = solve_part1(lines);
    let part2: String = solve_part2(lines);

    (part1, part2)
}

fn solve_part1(lines: &[String]) -> String {
    count_trees(lines, 3, 1).to_string()
}

fn solve_part2(lines: &[String]) -> String {
    (count_trees(lines, 1, 1)
        * count_trees(lines, 3, 1)
        * count_trees(lines, 5, 1)
        * count_trees(lines, 7, 1)
        * count_trees(lines, 1, 2))
    .to_string()
}

fn count_trees(lines: &[String], right: usize, down: usize) -> usize {
    let mut right_index: usize = 0;
    let mut down_index: usize = 0;

    lines
        .iter()
        .filter(|line| {
            let result = (**line).chars().cycle().nth(right_index).unwrap() == '#'
                && (down_index % down == 0);
            right_index += right;
            down_index += 1;
            result
        })
        .count()
}
