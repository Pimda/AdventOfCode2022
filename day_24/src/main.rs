type Bounds = (i32, i32);

pub mod collections;
use std::{collections::HashMap, mem};

use collections::PriorityQueue;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    string
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_1(map: &[Vec<char>]) -> i32 {
    let bounds = determine_bounds(map);
    let start_point = (1, 0);
    let end_point = (bounds.0 - 2, bounds.1 - 1);

    traverse(start_point, end_point, 0, map) 
}

fn part_2(map: &[Vec<char>]) -> i32 {
    let bounds = determine_bounds(map);
    let mut start_point = (1, 0);
    let mut end_point = (bounds.0 - 2, bounds.1 - 1);

    let mut total_turns = 0;

    for _ in 0..3{
        let turns = traverse(start_point, end_point, total_turns, map);

        total_turns += turns;
        mem::swap(&mut start_point, &mut end_point);
    }

    total_turns
}

fn traverse(
    start_point: (i32, i32),
    end_point: (i32, i32),
    start_turn: i32,
    map: &[Vec<char>],
) -> i32 {
    let bounds = determine_bounds(map);
    let directions = get_directions();

    let shortest_distance = manhattan_distance(start_point, end_point) as i32;
    let mut seen_options = HashMap::new();
    let mut collection = PriorityQueue::new();
    collection.push((start_point, start_turn), shortest_distance);
    loop {
        if let Some((position, turn)) = collection.pop_lowest() {

            for direction in directions {
                let target_position = add_tuples(position, direction);

                if target_position == end_point {
                    return turn - start_turn;
                }

                if is_in_bounds(target_position, bounds)
                    && is_empty(target_position, map, turn, bounds)
                {
                    let next_turn = turn + 1;
                    
                    if seen_options.contains_key(&(target_position, next_turn)) {
                        continue;
                    }
                    
                    let remaining_distance = manhattan_distance(target_position, end_point) as i32;

                    collection.push((target_position, next_turn), next_turn + remaining_distance);
                    seen_options.insert((target_position, next_turn), ());
                }
            }
        } else {
            panic!("No result found!")
        }
    }
}

fn is_empty(target_position: (i32, i32), map: &[Vec<char>], turn: i32, bounds: Bounds) -> bool {
    // calculate relative to the inner rectangle
    let normalized_x = target_position.0 - 1;
    let normalized_y = target_position.1 - 1;
    let inner_width = bounds.0 - 2;
    let inner_height = bounds.1 - 2;

    // find the positions a storm would have come from to reach this position
    let source_right = (positive_mod(normalized_x - turn, inner_width) + 1) as usize;
    let source_left = (positive_mod(normalized_x + turn, inner_width) + 1) as usize;
    let source_down = (positive_mod(normalized_y - turn, inner_height) + 1) as usize;
    let source_up = (positive_mod(normalized_y + turn, inner_height) + 1) as usize;

    map[target_position.1 as usize][source_right] != '>'
        && map[target_position.1 as usize][source_left] != '<'
        && map[source_down][target_position.0 as usize] != 'v'
        && map[source_up][target_position.0 as usize] != '^'
        && map[target_position.1 as usize][target_position.0 as usize] != '#'
}

fn positive_mod(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}

fn is_in_bounds(target_position: (i32, i32), bounds: Bounds) -> bool {
    target_position.0 >= 0
        && target_position.1 >= 0
        && target_position.0 < bounds.0
        && target_position.1 < bounds.1
}

fn get_directions() -> [(i32, i32); 5] {
    [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)]
}

fn determine_bounds(input: &[Vec<char>]) -> Bounds {
    let width = input[0].len();
    let height = input.len();
    (width as i32, height as i32)
}

fn add_tuples(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn manhattan_distance(lhs: (i32, i32), rhs: (i32, i32)) -> u32 {
    ((lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 18);
    }

    #[test]
    fn part_2_works() {
        let input = read_input("test.txt");
        assert_eq!(part_2(&input), 54);
    }
}
