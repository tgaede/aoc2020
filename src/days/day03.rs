pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let result = count_trees(input, 3, 1);
    println!("part 1: {}", result);
}

fn solve_part2(input: &str) {
    let result = count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2);

    println!("part 2: {}", result);
}

fn count_trees(input: &str, right: usize, down: usize) -> usize {
    let mut right_index: usize = 0;
    let mut down_index: usize = 0;

    input
        .split("\n")
        .filter(|line| {
            let result = (**line).chars().cycle().nth(right_index).unwrap() == '#'
                && (down_index % down == 0);
            right_index += right;
            down_index += 1;
            result
        })
        .count()
}
