use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FIELDS_REGEX: Vec<regex::Regex> = vec![
        Regex::new(r"\bbyr:(19[2-9]\d|200[0-2])\b"),
        Regex::new(r"\biyr:20(1\d|20)\b"),
        Regex::new(r"\beyr:20(2\d|30)\b"),
        Regex::new(r"\bhgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b"),
        Regex::new(r"\bhcl:#[0-9a-f]{6}\b"),
        Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b"),
        Regex::new(r"\bpid:\d{9}\b")
    ]
    .into_iter()
    .filter_map(|reg| reg.ok())
    .collect();
}

pub fn solve(input: &str) {
    let passports: Vec<_> = input.split("\n\n").collect();
    let count_present = passports
        .iter()
        .filter(|passport| all_fields_present(passport))
        .count();
    let count_valid = passports
        .iter()
        .filter(|passport| all_fields_present(passport))
        .filter(|passport| all_fields_valid(passport))
        .count();

    println!("part 1: {}", count_present);
    println!("part 2: {}", count_valid);
}

fn all_fields_present(passport: &str) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    fields.iter().all(|field| {
        passport
            .split_ascii_whitespace()
            .any(|passport_field| passport_field.starts_with(field))
    })
}

fn all_fields_valid(passport: &str) -> bool {
    FIELDS_REGEX.iter().all(|field| {
        passport
            .split_ascii_whitespace()
            .any(|passport_field| field.is_match(passport_field))
    })
}
