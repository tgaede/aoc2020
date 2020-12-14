pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let mut input_vec: Vec<usize> = input
        .split("\n")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    input_vec.sort();
    println!("part 1: {}", calc_joltage_differences(&input_vec));
}

fn calc_joltage_differences(data: &Vec<usize>) -> usize {
    let mut differences: Vec<usize> = vec![0; 4];
    let mut previous: usize = 0;

    data.iter().for_each(|x| {
        differences[x - previous] += 1;
        previous = *x;
    });

    differences[3] += 1; // device is always +3 after last adapter

    differences[1] * differences[3]
}

fn solve_part2(input: &str) {
    let mut input_vec: Vec<usize> = input
        .split("\n")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    input_vec.push(0);
    input_vec.push(input_vec.iter().max().unwrap() + 3);
    input_vec.sort();
    println!("part 2: {}", count_arrangements(&input_vec));
}

fn count_arrangements(data: &Vec<usize>) -> usize {
    let mut running: Vec<usize> = vec![0; data.len()];
    let mut skip: Vec<usize> = vec![0; data.len()];
    let mut count: Vec<usize> = vec![0; data.len()];
    count[1] = 1;

    let mut i: usize = 2;
    while i < data.len() {
        if i >= 3 && (data[i] - data[i - 3]) <= 3 {
            skip[i] += 1;
        }
        if i >= 2 && (data[i] - data[i - 2]) <= 3 {
            skip[i] += 1;
        }

        if skip[i] > 0 && skip[i - 1] + skip[i - 2] == 3 {
            running[i] = running[i - 1] + running[i - 2];
        } else if skip[i] > 0 {
            running[i] = count[i - 1];
        }
        count[i] = count[i - 1] + running[i];

        i += 1;
    }

    count[count.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn print_arrangements(arr: &Vec<HashSet<usize>>, data: &Vec<usize>) {
        for row_index in 0..arr.len() {
            let mut line: String = String::new();
            line += format!("{})\t", row_index).as_str();
            for i in 0..data.len() {
                if arr[row_index].contains(&data[i]) {
                    line += format!("{}", data[i]).as_str();
                }
                line += format!("\t").as_str();
            }
            println!("{}", line);
        }
    }

    fn get_arrangements(data: &Vec<usize>) -> Vec<HashSet<usize>> {
        let mut arrangements: Vec<HashSet<usize>> = Vec::new();
        let all: HashSet<usize> = HashSet::from_iter(data.iter().cloned());
        arrangements.push(all.clone());

        let mut running: Vec<usize> = vec![0; data.len()];
        let mut skip: Vec<usize> = vec![0; data.len()];
        let mut count: Vec<usize> = vec![0; data.len()];

        count[1] = 1;

        let mut i: usize = 2;

        println!("i)\tprev\tnew\ttot\tskip\trun\tcnt");

        while i < data.len() {
            let mut new_options: Vec<HashSet<usize>> = Vec::new();
            for item in &arrangements {
                if i >= 3
                    && item.contains(&data[i - 2])
                    && item.contains(&data[i - 3])
                    && (data[i] - data[i - 3]) <= 3
                {
                    let mut option1: HashSet<usize> = item.clone();
                    option1.remove(&data[i - 1]);
                    option1.remove(&data[i - 2]);
                    new_options.push(option1.clone());
                }

                if i >= 2
                    && item.contains(&data[i - 1])
                    && item.contains(&data[i - 2])
                    && (data[i] - data[i - 2]) <= 3
                {
                    let mut option1: HashSet<usize> = item.clone();
                    option1.remove(&data[i - 1]);
                    new_options.push(option1.clone());
                }
            }

            if i >= 3 && (data[i] - data[i - 3]) <= 3 {
                skip[i] += 1;
            }
            if i >= 2 && (data[i] - data[i - 2]) <= 3 {
                skip[i] += 1;
            }

            if skip[i] > 0 && skip[i - 1] + skip[i - 2] == 3 {
                running[i] = running[i - 1] + running[i - 2];
            } else if skip[i] > 0 {
                running[i] = count[i - 1];
            }
            count[i] = count[i - 1] + running[i];

            println!(
                "{})\t{}\t{}\t{}\t{}\t{}\t{}",
                i,
                arrangements.len(),
                new_options.len(),
                arrangements.len() + new_options.len(),
                skip[i],
                running[i],
                count[i]
            );

            if !new_options.is_empty() {
                // print_arrangements(&new_options, &data);
                arrangements.append(&mut new_options);
            }
            i += 1;
        }

        println!("");
        println!("all items:");
        print_arrangements(&arrangements, &data);
        arrangements
    }

    #[test]
    fn test_day10_part1_1() {
        let mut input: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(calc_joltage_differences(&input), 35);
    }

    #[test]
    fn test_day10_part1_2() {
        let mut input: Vec<usize> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.sort();
        assert_eq!(calc_joltage_differences(&input), 220);
    }

    #[test]
    fn test_day10_part2_1() {
        let mut input: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 0, 22];
        // let mut input: Vec<usize> = vec![1, 4, 5, 6, 7, 10];
        input.sort();
        assert_eq!(count_arrangements(&input), 8);
        println!("");
    }

    #[test]
    fn test_day10_part2_2() {
        let mut input: Vec<usize> = vec![0, 1, 4, 5, 6, 7, 8, 10];
        input.sort();
        assert_eq!(count_arrangements(&input), 11);
        println!("");
    }
    #[test]
    fn test_day10_part2_3() {
        let mut input: Vec<usize> = vec![0, 3, 6, 7, 10, 11, 12, 13, 16];
        input.sort();
        assert_eq!(count_arrangements(&input), 10);
        println!("");
    }

    #[test]
    fn test_day10_part2_4() {
        let mut input: Vec<usize> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 52,
        ];
        input.sort();
        let arrangements: Vec<HashSet<usize>> = get_arrangements(&input);
        let tests: Vec<Vec<usize>> = vec![
            vec![
                0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 47, 48, 49, 52,
            ],
            vec![
                0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 47, 49, 52,
            ],
            vec![
                0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 48, 49, 52,
            ],
            vec![
                0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 49, 52,
            ],
            vec![
                0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 47, 48, 49, 52,
            ],
            vec![
                0, 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 46, 48, 49,
                52,
            ],
            vec![
                0, 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 46, 49, 52,
            ],
            vec![
                0, 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 47, 48, 49,
                52,
            ],
            vec![
                0, 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 47, 49, 52,
            ],
            vec![
                0, 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 48, 49, 52,
            ],
        ];

        for i in 0..tests.len() {
            println!(
                "test {}: {}",
                i,
                arrangements.contains(&HashSet::from_iter(tests[i].iter().cloned()))
            );
            assert_eq!(
                arrangements.contains(&HashSet::from_iter(tests[i].iter().cloned())),
                true
            );
        }

        assert_eq!(arrangements.len(), 19208);
        println!("");
    }
}
