use std::{mem, collections::HashMap};
use aoc_helper::{collections::PriorityQueue, vec2d::Vec2D, math, navigation};

extern crate aoc_helper;

type Bounds = Vec2D;

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
    let start_point = Vec2D::new(1, 0);
    let end_point = Vec2D::new(bounds.x - 2, bounds.y - 1);

    traverse(start_point, end_point, 0, map) 
}

fn part_2(map: &[Vec<char>]) -> i32 {
    let bounds = determine_bounds(map);
    let mut start_point = Vec2D::new(1, 0);
    let mut end_point = Vec2D::new(bounds.x - 2, bounds.y - 1);

    let mut total_turns = 0;

    for _ in 0..3{
        let turns = traverse(start_point, end_point, total_turns, map);

        total_turns += turns;
        mem::swap(&mut start_point, &mut end_point);
    }

    total_turns
}

fn traverse(
    start_point: Vec2D,
    end_point: Vec2D,
    start_turn: i32,
    map: &[Vec<char>],
) -> i32 {
    let bounds = determine_bounds(map);
    let directions = navigation::get_adjecent_directions_including_self();

    let shortest_distance = start_point.manhattan_distance(end_point) as i32;
    let mut seen_options = HashMap::new();
    let mut collection = PriorityQueue::new();
    collection.push((start_point, start_turn), shortest_distance);
    loop {
        if let Some((position, turn)) = collection.pop_lowest() {

            for direction in directions {
                let target_position = position + direction;

                if target_position == end_point {
                    return turn - start_turn;
                }

                if target_position.is_in_bounds(bounds)
                    && is_empty(target_position, map, turn, bounds)
                {
                    let next_turn = turn + 1;
                    
                    if seen_options.contains_key(&(target_position, next_turn)) {
                        continue;
                    }
                    
                    let remaining_distance = target_position.manhattan_distance(end_point) as i32;

                    collection.push((target_position, next_turn), next_turn + remaining_distance);
                    seen_options.insert((target_position, next_turn), ());
                }
            }
        } else {
            panic!("No result found!")
        }
    }
}

fn is_empty(target_position: Vec2D, map: &[Vec<char>], turn: i32, bounds: Bounds) -> bool {
    // calculate relative to the inner rectangle
    let normalized_x = target_position.x - 1;
    let normalized_y = target_position.y - 1;
    let inner_width = bounds.x - 2;
    let inner_height = bounds.y - 2;

    // find the positions a storm would have come from to reach this position
    let source_right = (math::positive_mod(normalized_x - turn, inner_width) + 1) as usize;
    let source_left = (math::positive_mod(normalized_x + turn, inner_width) + 1) as usize;
    let source_down = (math::positive_mod(normalized_y - turn, inner_height) + 1) as usize;
    let source_up = (math::positive_mod(normalized_y + turn, inner_height) + 1) as usize;

    map[target_position.y as usize][source_right] != '>'
        && map[target_position.y as usize][source_left] != '<'
        && map[source_down][target_position.x as usize] != 'v'
        && map[source_up][target_position.x as usize] != '^'
        && map[target_position.y as usize][target_position.x as usize] != '#'
}

fn determine_bounds(input: &[Vec<char>]) -> Bounds {
    let width = input[0].len();
    let height = input.len();
    Vec2D::new(width as i32, height as i32)
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
