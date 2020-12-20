use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^([\w\s]+): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
}

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let mut input_section_iter = input.trim().split("\n\n");

    let rules: HashMap<String, HashSet<u32>> = parse_rules(input_section_iter.next().unwrap());
    let _your_ticket: Vec<u32> = parse_your_ticket(input_section_iter.next().unwrap());
    let nearby_tickets: Vec<Vec<u32>> = parse_nearby_tickets(input_section_iter.next().unwrap());
    let mut invalid_sum: u32 = 0;

    for ticket in &nearby_tickets {
        for item in ticket {
            if !rules.iter().any(|(_k, v)| v.contains(item)) {
                invalid_sum += item;
            }
        }
    }

    invalid_sum as usize
}

fn parse_rules(input: &str) -> HashMap<String, HashSet<u32>> {
    let mut rules: HashMap<String, HashSet<u32>> = HashMap::new();

    input.split("\n").for_each(|line| {
        let matches = REG.captures(line).unwrap();
        let rule_name = matches.get(1).unwrap().as_str();
        let min: u32 = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let max: u32 = matches.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let min2: u32 = matches.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let max2: u32 = matches.get(5).unwrap().as_str().parse::<u32>().unwrap();
        let mut set: HashSet<u32> = HashSet::new();

        for i in min..(max + 1) {
            set.insert(i);
        }
        for i in min2..(max2 + 1) {
            set.insert(i);
        }
        rules.insert(rule_name.to_string(), set);
    });

    rules
}

fn parse_your_ticket(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn parse_nearby_tickets(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .skip(1)
        .map(|line| line.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect()
}

fn solve_part2(input: &str) -> usize {
    let mut input_section_iter = input.trim().split("\n\n");

    let rules: HashMap<String, HashSet<u32>> = parse_rules(input_section_iter.next().unwrap());
    let your_ticket: Vec<u32> = parse_your_ticket(input_section_iter.next().unwrap());
    let nearby_tickets: Vec<Vec<u32>> = parse_nearby_tickets(input_section_iter.next().unwrap());
    let mut valid_fields: Vec<HashSet<String>> =
        vec![rules.keys().cloned().collect(); your_ticket.len()];

    // first remove totally invalid tickets
    let valid_nearby_tickets: Vec<Vec<u32>> = nearby_tickets
        .iter()
        .filter(|ticket| {
            !ticket
                .iter()
                .any(|item| !rules.iter().any(|(_k, v)| v.contains(item)))
        })
        .cloned()
        .collect();

    // start with every rule is valid for every slot. remove possibilities that aren't valid.
    for ticket_number in 0..valid_nearby_tickets.len() {
        let ticket = &valid_nearby_tickets[ticket_number];
        for i in 0..ticket.len() {
            for (rule, valid_rule_numbers) in &rules {
                if !valid_rule_numbers.contains(&ticket[i]) {
                    valid_fields[i].remove(rule.as_str());
                }
            }
        }
    }

    // find initial slots that have only 1 field possibility
    let mut done_fields: Vec<String> = Vec::new();
    for fields in &valid_fields {
        if fields.len() == 1 {
            done_fields.push(fields.iter().next().unwrap().clone());
        }
    }

    // remove confirmed fields from others recursively
    while !done_fields.is_empty() {
        let field_to_remove: String = done_fields.pop().unwrap();
        for entry in &mut valid_fields {
            if entry.len() != 1 {
                entry.remove(&field_to_remove);
                if entry.len() == 1 {
                    done_fields.push(entry.iter().next().unwrap().clone())
                }
            }
        }
    }

    // get result
    let mut result: usize = 1;
    for i in 0..valid_fields.len() {
        if valid_fields[i].len() == 1
            && valid_fields[i]
                .iter()
                .next()
                .unwrap()
                .starts_with("departure")
        {
            result *= your_ticket[i] as usize;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_part1() {
        assert_eq!(solve_part1("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"), 71);
    }
}
