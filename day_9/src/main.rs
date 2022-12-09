use std::collections::HashMap;

fn main() {
    let input = parse_input("input.txt");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn parse_input(filename: &str) -> Vec<(char, u32)> {
    let input = std::fs::read_to_string(filename).expect("Could not read file");

    input
        .lines()
        .map(|l| {
            if let [dir, len] = l.split(' ').collect::<Vec<&str>>()[..] {
                (dir.chars().next().unwrap(), len.parse().unwrap())
            } else {
                panic!("invalid command")
            }
        })
        .collect()
}

fn part_1(moves: &Vec<(char, u32)>) -> usize {
    simulate(moves, 2)
}

fn part_2(moves: &Vec<(char, u32)>) -> usize {
    simulate(moves, 10)
}

fn simulate(moves: &Vec<(char, u32)>, count: usize) -> usize {
    let mut knots = vec![(0, 0); count - 1];
    let mut head = (0, 0);

    let mut visited_positions: HashMap<(i32, i32), ()> = HashMap::new();
    visited_positions.insert(*knots.last().unwrap(), ());

    for (direction, length) in moves {
        for _ in 0..*length {
            match direction {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                _ => panic!("invalid direction"),
            }

            let mut prev_knot = head;

            for mut knot in knots.iter_mut() {
                let x_diff: i32 = prev_knot.0 - knot.0;
                let y_diff: i32 = prev_knot.1 - knot.1;

                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    knot.0 += x_diff.signum();
                    knot.1 += y_diff.signum();
                }

                prev_knot = *knot;
            }

            visited_positions
                .entry(*knots.last().unwrap())
                .or_insert_with(|| ());
        }
    }

    visited_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = parse_input("test.txt");
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn part_2_works() {
        let input = parse_input("test.txt");
        assert_eq!(part_2(&input), 1);
    }
}
