pub fn solve(input: &str) {
    solve_part1(input);
    solve_part2(input);
}

fn solve_part1(input: &str) {
    let result = input
        .split("\n")
        .map(|seat| {
            seat.chars().fold(0, |seat_id, c| match c {
                'F' | 'L' => (seat_id << 1) | 0,
                'B' | 'R' => (seat_id << 1) | 1,
                _ => unreachable!(),
            })
        })
        .max()
        .expect("iterator isn't empty");

    println!("part 1: {}", result);
}

fn solve_part2(input: &str) {
    let mut seat_ids: Vec<i32> = input
        .split("\n")
        .map(|seat| {
            seat.chars().fold(0, |seat_id, c| match c {
                'F' | 'L' => (seat_id << 1) | 0,
                'B' | 'R' => (seat_id << 1) | 1,
                _ => unreachable!(),
            })
        })
        .collect();

    seat_ids.sort();

    let mut my_seat: i32 = 0;
    for seats in seat_ids.as_slice().windows(2) {
        if seats[1] - seats[0] > 1 {
            my_seat = seats[0] + 1;
            break;
        }
    }

    println!("part 2: {}", my_seat);
}
