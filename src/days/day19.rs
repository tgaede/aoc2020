use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> i64 {
    let mut input_iter = input.trim().split("\n\n");
    let (number_rules, char_rules) = parse_rules(input_iter.next().unwrap());
    let mut cache: HashMap<usize, HashSet<String>> = HashMap::new();
    let rule_zero_options = get_rule_options(0, &number_rules, &char_rules, &mut cache);

    input_iter
        .next()
        .unwrap()
        .split("\n")
        .filter(|message| {
            rule_zero_options
                .iter()
                .any(|rule| (*message) == rule.as_str())
        })
        .count() as i64
}

fn solve_part2(input: &str) -> i64 {
    let mut input_iter = input.trim().split("\n\n");
    let (number_rules, char_rules) = parse_rules(input_iter.next().unwrap());

    // setup the cache
    let mut cache: HashMap<usize, HashSet<String>> = HashMap::new();
    for (rule_number, _o) in &number_rules {
        get_rule_options(*rule_number, &number_rules, &char_rules, &mut cache);
    }
    let rule_31_options: HashSet<String> = cache.get(&31).unwrap().clone();
    let rule_42_options: HashSet<String> = cache.get(&42).unwrap().clone();
    let rule_31_len = rule_31_options.iter().next().unwrap().len();
    let rule_42_len = rule_42_options.iter().next().unwrap().len();

    // format goes like this:
    // 42{n}31{<n}
    let mut count: i64 = 0;
    for message in input_iter.next().unwrap().split("\n") {
        let mut i: usize = message.len() - rule_31_len;
        let mut count_31: i64 = 0;
        let mut count_42: i64 = 0;
        'inner: loop {
            if rule_31_options
                .iter()
                .any(|option| *option.as_str() == message[i..(i + rule_31_len)])
                && count_42 == 0
            {
                count_31 += 1;
                if i >= rule_31_len {
                    i -= rule_31_len;
                } else {
                    break 'inner;
                }
            } else if rule_42_options
                .iter()
                .any(|option| *option.as_str() == message[i..(i + rule_42_len)])
                && count_31 > 0
            {
                count_42 += 1;
                if i >= rule_42_len {
                    i -= rule_42_len;
                } else {
                    break 'inner;
                }
            } else {
                break 'inner;
            }
        }

        if i == 0 && count_42 > count_31 {
            count += 1;
        }
    }

    count
}

fn parse_rules(input: &str) -> (HashMap<usize, Vec<Vec<usize>>>, HashMap<usize, char>) {
    let mut number_rules: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut char_rules: HashMap<usize, char> = HashMap::new();

    for line in input.split("\n") {
        let mut i: usize = line.find(":").unwrap();
        let rule_number: usize = line.get(0..i).unwrap().parse().unwrap();
        i += 2;

        if line.contains("\"") {
            char_rules.insert(rule_number, line.chars().nth(i + 1).unwrap());
        } else if line.contains("|") {
            let left_side: Vec<usize> = line
                .get(i..)
                .unwrap()
                .split(" | ")
                .next()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            let right_side: Vec<usize> = line
                .get(i..)
                .unwrap()
                .rsplit(" | ")
                .next()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            number_rules.insert(rule_number, vec![left_side, right_side]);
        } else {
            // regular rule
            let rule: Vec<usize> = line
                .get(i..)
                .unwrap()
                .split(" | ")
                .next()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            number_rules.insert(rule_number, vec![rule]);
        }
    }

    (number_rules, char_rules)
}

fn get_rule_options(
    rule_number: usize,
    number_rules: &HashMap<usize, Vec<Vec<usize>>>,
    char_rules: &HashMap<usize, char>,
    cache: &mut HashMap<usize, HashSet<String>>,
) -> HashSet<String> {
    let mut options: HashSet<String> = HashSet::new();

    if char_rules.contains_key(&rule_number) {
        options.insert(char_rules.get(&rule_number).unwrap().to_string());
    } else if number_rules.contains_key(&rule_number) {
        for option in number_rules.get(&rule_number).unwrap() {
            let mut current_options: HashSet<String> = HashSet::new();
            let mut combined_options: HashSet<String> = HashSet::new();

            for rule in option {
                let child_options: HashSet<String>;
                if cache.contains_key(rule) {
                    child_options = cache.get(rule).unwrap().clone();
                } else {
                    child_options = get_rule_options(*rule, number_rules, char_rules, cache);
                }

                for child_option in &child_options {
                    for current_option in &current_options {
                        let s: String = current_option.to_owned() + child_option;
                        combined_options.insert(s);
                    }
                    if current_options.is_empty() {
                        for o in &child_options {
                            combined_options.insert(o.to_string());
                        }
                    }
                }
                current_options.clear();
                for o in &combined_options {
                    current_options.insert(o.to_string());
                }
                combined_options.clear();
            }

            for item in &current_options {
                options.insert(item.clone());
            }
        }
    }

    if !cache.contains_key(&rule_number) {
        cache.insert(rule_number, options.clone());
    }

    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_part1() {
        assert_eq!(solve_part1("0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb"), 2);
        assert_eq!(solve_part1("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), 3)
    }

    #[test]
    fn test_day19_part2() {
        assert_eq!(solve_part2("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), 12)
    }
}
