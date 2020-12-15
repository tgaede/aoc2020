const WEST: char = 'W';
const NORTH: char = 'N';
const EAST: char = 'E';
const SOUTH: char = 'S';

const LEFT: char = 'L';
const RIGHT: char = 'R';
const FORWARD: char = 'F';

#[derive(Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
    direction: char,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            direction: 'E',
        }
    }
}

pub fn solve(input: &str) {
    println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let mut p: Position = Position::new();

    input
        .trim()
        .split("\n")
        .for_each(|line| p = process_line_part1(line, &p));

    (p.x.abs() + p.y.abs()) as usize
}

fn process_line_part1(line: &str, p: &Position) -> Position {
    let mut new_p: Position = p.clone();
    let mut action: char = line.chars().next().unwrap();
    let distance: i64 = line[1..].parse().unwrap();

    if action == FORWARD {
        action = p.direction;
    }

    match action {
        WEST => new_p.x -= distance,
        EAST => new_p.x += distance,
        NORTH => new_p.y += distance,
        SOUTH => new_p.y -= distance,
        RIGHT => new_p.direction = turn_ship(p, RIGHT, distance),
        LEFT => new_p.direction = turn_ship(p, LEFT, distance),
        _ => unreachable!(),
    };

    new_p
}

fn turn_ship(p: &Position, turn_dir: char, degrees: i64) -> char {
    let turn_count = degrees.abs() / 90;
    let mut current_dir: char = p.direction;
    for _ in 0..turn_count {
        match (current_dir, turn_dir) {
            (WEST, RIGHT) => current_dir = NORTH,
            (WEST, LEFT) => current_dir = SOUTH,
            (EAST, RIGHT) => current_dir = SOUTH,
            (EAST, LEFT) => current_dir = NORTH,
            (SOUTH, RIGHT) => current_dir = WEST,
            (SOUTH, LEFT) => current_dir = EAST,
            (NORTH, RIGHT) => current_dir = EAST,
            (NORTH, LEFT) => current_dir = WEST,
            _ => unreachable!(),
        }
    }

    current_dir
}

fn solve_part2(input: &str) -> usize {
    let mut ship: Position = Position::new();
    let mut waypoint: Position = Position::new();
    waypoint.x = 10;
    waypoint.y = 1;

    input.trim().split("\n").for_each(|line| {
        let p = process_line_part2(line, &ship, &waypoint);
        ship = p.0;
        waypoint = p.1;
    });

    (ship.x.abs() + ship.y.abs()) as usize
}

fn process_line_part2(line: &str, ship: &Position, waypoint: &Position) -> (Position, Position) {
    let mut new_ship: Position = ship.clone();
    let mut new_waypoint: Position = waypoint.clone();

    let action: char = line.chars().next().unwrap();
    let distance: i64 = line[1..].parse().unwrap();

    match action {
        WEST => new_waypoint.x -= distance,
        EAST => new_waypoint.x += distance,
        NORTH => new_waypoint.y += distance,
        SOUTH => new_waypoint.y -= distance,
        RIGHT => new_waypoint = rotate_waypoint(&waypoint, RIGHT, distance),
        LEFT => new_waypoint = rotate_waypoint(&waypoint, LEFT, distance),
        FORWARD => {
            new_ship.x += waypoint.x * distance;
            new_ship.y += waypoint.y * distance;
        }
        _ => unreachable!(),
    };

    (new_ship, new_waypoint)
}

fn rotate_waypoint(waypoint: &Position, turn_dir: char, degrees: i64) -> Position {
    let mut new_waypoint: Position = waypoint.clone();
    let turn_count = degrees.abs() / 90;

    for _ in 0..turn_count {
        let temp = new_waypoint.x;
        new_waypoint.x = new_waypoint.y;
        new_waypoint.y = temp;
        if turn_dir == RIGHT {
            new_waypoint.y = -new_waypoint.y;
        } else if turn_dir == LEFT {
            new_waypoint.x = -new_waypoint.x;
        }
    }

    new_waypoint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let input: &str = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(solve_part1(input), 25);
    }

    #[test]
    fn test_day12_part2() {
        let input: &str = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(solve_part2(input), 286);
    }
}
