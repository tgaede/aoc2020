use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_bag(line: &str) -> (String, HashMap<String, usize>) {
    let mut parts = line.split(" bags contain ");

    let parent_color = parts.next().unwrap().to_string();
    let mut rules: HashMap<String, usize> = HashMap::new();
    println!("[{}] - start", parent_color);

    let contents = parts.next().unwrap().split(", ");
    for item in contents {
        println!("[{}] parsing[{}]", parent_color, item);
        if item.starts_with("no other bag") {
            break;
        }

        let mut contents_parts = item.split_ascii_whitespace();
        let quantity: usize = contents_parts.next().unwrap().parse().unwrap();
        let color: String =
            contents_parts.next().unwrap().to_owned() + " " + contents_parts.next().unwrap();
        println!("[{}] qantity[{}] color [{}]", parent_color, quantity, color);
        rules.insert(color, quantity);
    }

    (parent_color, rules)
}

pub fn solve(input: &str) {
    //    let test_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let bags: HashMap<String, HashMap<String, usize>> =
        input.lines().map(|line| parse_bag(line)).collect();
    let mut final_set: HashSet<String> = HashSet::new();
    let mut previous_set: HashSet<String> = HashSet::new();

    previous_set.insert(String::from("shiny gold"));

    loop {
        let mut current_set: HashSet<String> = bags
            .iter()
            .filter(|&(_k, v)| previous_set.iter().any(|i| v.contains_key(i)))
            .map(|(k, _v)| k.clone())
            .collect();

        current_set = current_set
            .difference(&final_set)
            .map(|x| x.clone())
            .collect::<HashSet<String>>();
        final_set.extend(current_set.iter().map(|x| x.clone()));

        println!("next_set: {:#?}", current_set);
        println!("previous_set: {:#?}", previous_set);
        println!("final_set: {:#?}", final_set);

        previous_set = current_set;
        if previous_set.is_empty() {
            break;
        }
    }

    println!("part 1: {:?}", final_set.len());
}

fn solve_part2(input: &str) {
    let rules: HashMap<String, HashMap<String, usize>> =
        input.lines().map(|line| parse_bag(line)).collect();

    let mut bag_queue: VecDeque<(String, usize)> = VecDeque::new();

    bag_queue.push_back((String::from("shiny gold"), 1));
    let mut count: usize = 0;

    loop {
        let bags: (String, usize) = bag_queue.pop_front().unwrap();
        count += bags.1;

        for (k, v) in rules.get(&bags.0).unwrap() {
            bag_queue.push_back((k.to_string(), v * bags.1));
        }

        if bag_queue.is_empty() {
            break;
        }
    }

    println!("part 2: {}", count - 1);
}
