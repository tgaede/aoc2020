type Map = Vec<Vec<char>>;

// const FLOOR: char = '.';
const FULL: char = '#';
const EMPTY: char = 'L';

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let mut map: Map = input
        .trim()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    while do_round_part1(&mut map) != 0 {}

    count_occupied_seats(&map)
}

fn count_occupied_seats(map: &Map) -> usize {
    map.iter()
        .map(|x| x.iter().filter(|x| **x == FULL).count())
        .sum()
}

// fn print_map(map: &Map) {
//     for row in 0..map.len() {
//         for col in 0..map[row].len() {
//             print!("{}", map[row][col]);
//         }
//         print!("\n");
//     }
//     print!("\n");
// }

fn do_round_part1(map: &mut Map) -> usize {
    let mut changes: Vec<(usize, usize, char)> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                EMPTY => {
                    if count_neighbors(&map, row, col) == 0 {
                        changes.push((row, col, FULL));
                    }
                }
                FULL => {
                    if count_neighbors(&map, row, col) >= 4 {
                        changes.push((row, col, EMPTY));
                    }
                }
                _ => {}
            }
        }
    }

    for change in &changes {
        map[change.0][change.1] = change.2;
    }

    changes.len()
}

fn count_neighbors(map: &Map, row: usize, col: usize) -> usize {
    let mut count: usize = 0;
    let min_row: usize = row.saturating_sub(1);
    let max_row: usize = (row + 1).min(map.len() - 1);
    let min_col: usize = col.saturating_sub(1);
    let max_col: usize = (col + 1).min(map[row].len() - 1);

    for temp_row in min_row..(max_row + 1) {
        for temp_col in min_col..(max_col + 1) {
            if !(temp_row == row && temp_col == col) && map[temp_row][temp_col] == FULL {
                count += 1;
            }
        }
    }

    count
}

fn solve_part2(input: &str) -> usize {
    let mut map: Map = input
        .trim()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    while do_round_part2(&mut map) != 0 {}

    count_occupied_seats(&map)
}

fn count_visible(map: &Map, row: usize, col: usize) -> usize {
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    directions
        .iter()
        .filter(|dir| {
            let mut current_row: usize = row;
            let mut current_col: usize = col;
            let mut see_occupied_chair: bool = false;
            while !see_occupied_chair {
                if current_row == 0 && dir.0 < 0
                    || current_row == map.len() - 1 && dir.0 > 0
                    || current_col == 0 && dir.1 < 0
                    || current_col == map[row].len() - 1 && dir.1 > 0
                {
                    break;
                }
                current_row = (current_row as i32 + dir.0) as usize;
                current_col = (current_col as i32 + dir.1) as usize;
                match map[current_row][current_col] {
                    FULL => see_occupied_chair = true,
                    EMPTY => break,
                    _ => {}
                }
            }

            see_occupied_chair
        })
        .count()
}

fn do_round_part2(map: &mut Map) -> usize {
    let mut changes: Vec<(usize, usize, char)> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                EMPTY => {
                    if count_visible(&map, row, col) == 0 {
                        changes.push((row, col, FULL));
                    }
                }
                FULL => {
                    if count_visible(&map, row, col) >= 5 {
                        changes.push((row, col, EMPTY));
                    }
                }
                _ => {}
            }
        }
    }

    for change in &changes {
        map[change.0][change.1] = change.2;
    }

    changes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1() {
        let input: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(solve_part1(input), 37);
    }

    #[test]
    fn test_day11_part2() {
        let input: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(solve_part2(input), 26);
    }
}
