pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let input_vec: Vec<usize> = input
        .split("\n")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    for i in 25..input_vec.len() {
        if !is_valid_number(&input_vec, i - 25, 25, input_vec[i]) {
            println!("part 1: {}", input_vec[i]);
            break;
        }
    }
}

fn is_valid_number(vec: &Vec<usize>, start: usize, len: usize, target: usize) -> bool {
    for i in start..(start + len - 1) {
        for j in (start + 1)..(start + len) {
            if vec[i] + vec[j] == target && i != j && vec[i] != vec[j] {
                return true;
            }
        }
    }

    false
}

fn solve_part2(input: &str) {
    let input_vec: Vec<usize> = input
        .split("\n")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let result = find_encryption_weakness(&input_vec, 105950735);
    println!("part 2: {}", result);
}

fn find_encryption_weakness(vec: &Vec<usize>, target: usize) -> usize {
    let mut count: usize;
    for i in 0..vec.len() {
        count = vec[i];
        for j in (i + 1)..vec.len() {
            count += vec[j];
            if count == target {
                return find_min_max_sum(vec, i, (j - i) + 1);
            } else if count > target {
                break;
            }
        }
    }

    unreachable!();
}

fn find_min_max_sum(vec: &Vec<usize>, start: usize, len: usize) -> usize {
    let min = vec.iter().skip(start).take(len).min().unwrap();
    let max = vec.iter().skip(start).take(len).max().unwrap();
    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09_part1() {
        let input: Vec<usize> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        for i in 5..input.len() {
            if input[i] != 127 {
                assert_eq!(is_valid_number(&input, i - 5, 5, input[i]), true);
            } else {
                assert_eq!(is_valid_number(&input, i - 5, 5, input[i]), false);
            }
        }
    }

    #[test]
    fn test_day09_part2() {
        let input: Vec<usize> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find_encryption_weakness(&input, 127), 62);
    }
}
