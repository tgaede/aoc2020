pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let mut numbers: Vec<u32> = input
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    numbers.sort();

    let mut result: u32 = 0;

    'outer: for x in 0..numbers.len() {
        for y in (x + 1)..numbers.len() {
            if (numbers[x] + numbers[y]) == 2020 {
                result = numbers[x] * numbers[y];
                break 'outer;
            } else if (numbers[x] + numbers[y]) > 2020 {
                break;
            }
        }
    }

    println!("part 1: {}", result);
}

fn solve_part2(input: &str) {
    let mut numbers: Vec<u32> = input
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    numbers.sort();

    let mut result: u32 = 0;

    'outer: for x in 0..numbers.len() {
        for y in (x + 1)..numbers.len() {
            for z in (y + 1)..numbers.len() {
                if (numbers[x] + numbers[y] + numbers[z]) == 2020 {
                    result = numbers[x] * numbers[y] * numbers[z];
                    break 'outer;
                } else if (numbers[x] + numbers[y] + numbers[z]) > 2020 {
                    break;
                }
            }
        }
    }

    println!("part 2: {}", result);
}
