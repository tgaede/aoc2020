pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let min_time: i64 = input.trim().split("\n").next().unwrap().parse().unwrap();
    let busses: Vec<i64> = input
        .trim()
        .split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .filter(|x| !(*x == "x"))
        .map(|x| x.parse().unwrap())
        .collect();

    // println!("min_time: {} busses: {:#?}", min_time, busses);

    let mut min_wait: i64 = std::i64::MAX;
    let mut bus_id: i64 = 0;

    for bus in busses {
        let current_wait = bus - min_time % bus;
        if current_wait < min_wait {
            min_wait = current_wait;
            bus_id = bus;
        }
    }

    (min_wait * bus_id) as usize
}

fn solve_part2(input: &str) -> usize {
    let busses: Vec<(i64, i64)> = input
        .trim()
        .split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_i, factor)| *factor != "x")
        .map(|(i, factor)| (i as i64, factor.parse::<i64>().unwrap()))
        .collect();
    let divisors: Vec<i64> = busses.iter().map(|(_i, factor)| *factor).collect();
    let mut remainders: Vec<i64> = busses.iter().map(|(i, factor)| factor - *i).collect();
    remainders[0] = 0;

    match chinese_remainder(&divisors, &remainders) {
        Some(solution) => {
            return solution as usize;
        }
        None => {
            return 0;
        }
    }
}

fn chinese_remainder(divisors: &Vec<i64>, remainders: &Vec<i64>) -> Option<i64> {
    let product: i64 = divisors.iter().product::<i64>();
    let mut sum: i64 = 0;

    for (&remainder, &divisor) in remainders.iter().zip(divisors) {
        let p = product / divisor;
        sum += remainder * mod_inv(p, divisor)? * p;
    }

    Some(sum % product)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1() {
        let input: &str = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(solve_part1(input), 295);
    }

    #[test]
    fn test_day13_part2_test1() {
        let input: &str = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(solve_part2(input), 1068781);
    }

    #[test]
    fn test_day13_part2_test2() {
        let input: &str = "939\n17,x,13,19";
        assert_eq!(solve_part2(input), 3417);
    }

    #[test]
    fn test_day13_part2_test3() {
        let input: &str = "939\n67,7,59,61";
        assert_eq!(solve_part2(input), 754018);
    }

    #[test]
    fn test_day13_part2_test4() {
        let input: &str = "939\n67,x,7,59,61";
        assert_eq!(solve_part2(input), 779210);
    }

    #[test]
    fn test_day13_part2_test5() {
        let input: &str = "939\n67,7,x,59,61";
        assert_eq!(solve_part2(input), 1261476);
    }

    #[test]
    fn test_day13_part2_test6() {
        let input: &str = "939\n1789,37,47,1889";
        assert_eq!(solve_part2(input), 1202161486);
    }
}
