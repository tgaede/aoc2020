use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^mem\[(\d+)] = (\d+)$").unwrap();
}

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let mut mask: (u64, u64, u64) = (0, 0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.split("\n") {
        if line.starts_with("mask") {
            mask = parse_mask(line);
        } else if line.starts_with("mem") {
            let (index, mut value) = parse_mem(line);
            value = value & mask.0;
            value = value | mask.1;
            memory.insert(index, value);
        }
    }

    memory.values().sum::<u64>() as usize
}

fn parse_mask(line: &str) -> (u64, u64, u64) {
    let mut zero_mask: u64 = 0;
    let mut one_mask: u64 = 0;
    let mut x_mask: u64 = 0;

    for c in line.rsplit(' ').next().unwrap().chars() {
        zero_mask = zero_mask.rotate_left(1);
        zero_mask = zero_mask | 1;
        one_mask = one_mask.rotate_left(1);
        x_mask = x_mask.rotate_left(1);

        match c {
            'X' => x_mask = x_mask | 1,
            '1' => one_mask = one_mask | 1,
            '0' => zero_mask = zero_mask & 0xFFFFFFFFFFFFFFFE,
            _ => unreachable!(),
        }
    }

    (zero_mask, one_mask, x_mask)
}

fn parse_mem(line: &str) -> (u64, u64) {
    let matches = REG.captures(line).unwrap();
    let index: u64 = matches.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let value: u64 = matches.get(2).unwrap().as_str().parse::<u64>().unwrap();

    (index, value)
}

fn solve_part2(input: &str) -> usize {
    let mut mask: (u64, u64, u64) = (0, 0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.split("\n") {
        if line.starts_with("mask") {
            mask = parse_mask(line);
        } else if line.starts_with("mem") {
            let (index, value) = parse_mem(line);
            let memory_indices: Vec<u64> = get_memory_indices(&mask, index);
            for mem_index in memory_indices {
                memory.insert(mem_index, value);
            }
        }
    }

    memory.values().sum::<u64>() as usize
}

fn get_memory_indices(mask: &(u64, u64, u64), index: u64) -> Vec<u64> {
    let mut memory_indices: Vec<u64>;
    let base_index: u64 = (index | mask.1) & !mask.2;

    if mask.2 == 0 {
        memory_indices = vec![base_index; 1];
        return memory_indices;
    }

    let combinations: Vec<u64> = (0..2u64.pow(mask.2.count_ones())).collect();
    memory_indices = vec![base_index; combinations.len()];
    let mut test_bit: u64;
    let mut combo_number: usize = 0;
    let mut combo_bit_index: u32;

    while combo_number < combinations.len() {
        test_bit = 1;
        combo_bit_index = 0;

        for index in 0..36 {
            if test_bit & mask.2 != 0 {
                memory_indices[combo_number] = memory_indices[combo_number]
                    | (combinations[combo_number].rotate_right(combo_bit_index) & 1)
                        .rotate_left(index);
                combo_bit_index += 1;
            }

            test_bit = test_bit.rotate_left(1);
        }

        combo_number += 1;
    }

    assert_eq!(memory_indices.len(), combinations.len());

    memory_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1() {
        let input: &str =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        assert_eq!(solve_part1(input), 165);
    }

    #[test]
    fn test_day14_part2() {
        let input: &str =
            "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        assert_eq!(solve_part2(input), 208);
    }
}
