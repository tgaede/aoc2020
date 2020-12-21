use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }
}

type World = HashMap<Point, char>;

const ACTIVE: char = '#';
const INACTIVE: char = '.';

pub fn solve(input: &str) {
    // println!("part 1: {}", solve_part1(input));
    println!("part 2: {}", solve_part2(input));
}

fn solve_part2(input: &str) -> usize {
    let mut world: World = parse_world(&input);

    for _i in 1..7 {
        // println!("After {} cycles:", i);
        world = cycle_world(&world);
        // print_world(&world);
    }

    world.iter().filter(|(_p, state)| **state == ACTIVE).count()
}

fn parse_world(input: &str) -> World {
    let mut world: World = HashMap::new();
    let mut p: Point = Point::new();

    input.trim().split("\n").for_each(|line| {
        p.x = 0;
        for c in line.chars() {
            match c {
                ACTIVE => {
                    world.insert(p, ACTIVE);
                }
                INACTIVE => {
                    world.insert(p, INACTIVE);
                }
                _ => unreachable!(),
            }
            p.x += 1;
        }
        p.y += 1;
    });

    world
}

// fn print_world(world: &World) {
//     let (min_point, max_point) = world_bounds(&world);
//     let mut p: Point = Point::new();
//
//     p.w = min_point.w;
//     while p.w < (max_point.w + 1) {
//         p.z = min_point.z;
//         while p.z < (max_point.z + 1) {
//             println!("z= {} w={}", p.z, p.w);
//             p.y = min_point.y;
//             while p.y < (max_point.y + 1) {
//                 let mut line: String = String::new();
//                 p.x = min_point.x;
//                 while p.x < (max_point.x + 1) {
//                     if world.contains_key(&p) {
//                         line.push(*world.get(&p).unwrap());
//                     } else {
//                         line.push(INACTIVE);
//                     }
//                     p.x += 1;
//                 }
//                 println!("{}", line);
//                 p.y += 1;
//             }
//             p.z += 1;
//             println!("");
//         }
//         p.w += 1;
//     }
// }
//
// fn world_bounds(world: &World) -> (Point, Point) {
//     let mut min_point: Point = Point::new();
//     let mut max_point: Point = Point::new();
//     for (p, _state) in world {
//         if p.x < min_point.x {
//             min_point.x = p.x;
//         }
//         if p.x > max_point.x {
//             max_point.x = p.x;
//         }
//         if p.y < min_point.y {
//             min_point.y = p.y;
//         }
//         if p.y > max_point.y {
//             max_point.y = p.y;
//         }
//         if p.z < min_point.z {
//             min_point.z = p.z;
//         }
//         if p.z > max_point.z {
//             max_point.z = p.z;
//         }
//         if p.w > max_point.w {
//             max_point.w = p.w;
//         }
//         if p.w < min_point.w {
//             min_point.w = p.w;
//         }
//     }
//
//     (min_point, max_point)
// }

fn cycle_world(world: &World) -> World {
    let mut changes: World = HashMap::new();
    let mut new_world: World = world.clone();

    for p in get_all_points(&world) {
        let count = count_active_neighbors(world, &p);
        if world.contains_key(&p) && *(world.get(&p).unwrap()) == ACTIVE {
            if !(count == 2 || count == 3) {
                changes.insert(p, INACTIVE);
            }
        } else if count == 3 {
            changes.insert(p, ACTIVE);
        }
    }

    for (p, s) in changes {
        new_world.insert(p, s);
    }

    new_world
}

fn is_neighbor(p: &Point, p2: &Point) -> bool {
    (p.x - p2.x).abs() <= 1
        && (p.y - p2.y).abs() <= 1
        && (p.z - p2.z).abs() <= 1
        && (p.w - p2.w).abs() <= 1
        && !(p.x == p2.x && p.y == p2.y && p.z == p2.z && p.w == p2.w)
}

fn get_neighbors(p: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let mut new_point: Point = Point::new();
    for x in p.x - 1..p.x + 2 {
        for y in p.y - 1..p.y + 2 {
            for z in p.z - 1..p.z + 2 {
                for w in p.w - 1..p.w + 2 {
                    if !(x == p.x && y == p.y && z == p.z && w == p.w) {
                        new_point.x = x;
                        new_point.y = y;
                        new_point.z = z;
                        new_point.w = w;
                        points.push(new_point);
                    }
                }
            }
        }
    }

    points
}

fn get_all_points(world: &World) -> Vec<Point> {
    let mut all_points: HashSet<Point> = HashSet::new();
    for (point, _state) in world {
        let new_points: Vec<Point> = get_neighbors(point);
        for new_point in new_points {
            all_points.insert(new_point);
        }
        all_points.insert(*point);
    }

    all_points.iter().cloned().collect()
}

fn count_active_neighbors(world: &World, point: &Point) -> usize {
    world
        .iter()
        .filter(|(p, state)| is_neighbor(*p, point) && **state == ACTIVE)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_part2() {
        assert_eq!(solve_part2(".#.\n..#\n###"), 848);
    }
}
