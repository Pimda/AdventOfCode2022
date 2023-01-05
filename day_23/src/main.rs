extern crate aoc_helper;

use aoc_helper::{navigation, vec2d::Vec2D};
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Vec2D> {
    let string = std::fs::read_to_string(filename).expect("File not found");

    let mut elves = vec![];

    for (y, line) in string.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push(Vec2D::new(x as i32, y as i32));
            }
        }
    }

    elves
}

fn part_1(elves: &[Vec2D]) -> i32 {
    let elves = simulate(elves, 10).0;

    empty_spaces_in_smallest_rectangle(elves)
}

fn part_2(elves: &[Vec2D]) -> usize {
    simulate(elves, usize::max_value()).1
}

fn empty_spaces_in_smallest_rectangle(elves: Vec<Vec2D>) -> i32 {
    let min_x = elves.iter().map(|e| e.x).min().unwrap();
    let max_x = elves.iter().map(|e| e.x).max().unwrap();
    let min_y = elves.iter().map(|e| e.y).min().unwrap();
    let max_y = elves.iter().map(|e| e.y).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    width * height - elves.len() as i32
}

fn simulate(elves: &[Vec2D], max_turns: usize) -> (Vec<Vec2D>, usize) {
    let mut directions = get_directions();
    let mut elves = elves.to_vec();
    for i in 0..max_turns {
        let mut proposals = HashMap::new();

        for &elf in elves.iter() {
            if are_surroundings_empty(&elves, elf) {
                proposals.insert(elf, elf);
                continue;
            }

            let target_position = get_first_free_position(elf, &directions, &elves);

            if let Some(new_position) = target_position {
                proposals.insert(elf, new_position);
                continue;
            }

            proposals.insert(elf, elf);
        }

        if proposals.iter().all(|(s, t)| *s == *t) {
            return (vec![], i + 1);
        }

        let mut new_positions = vec![];

        for (source, target) in proposals.iter() {
            if proposals.iter().filter(|(_, t)| *t == target).count() == 1 {
                new_positions.push(*target);
            } else {
                new_positions.push(*source);
            }
        }

        elves = new_positions;

        //print(height, width, &elves);

        let item = directions.pop_front().unwrap();
        directions.push_back(item);
    }
    (elves, 0)
}

fn _print(height: &i32, width: &i32, elves: &[Vec2D]) {
    for y in 0..*height {
        for x in 0..*width {
            if elves.contains(&Vec2D::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    //use std::io::stdin;
    //stdin().read_line(&mut "".to_owned());
}

fn get_first_free_position(
    point: Vec2D,
    directions: &VecDeque<Vec2D>,
    elves: &[Vec2D],
) -> Option<Vec2D> {
    for direction in directions {
        let target_position = point + *direction;

        if direction.x != 0 {
            let top = Vec2D::new(target_position.x, target_position.y - 1);
            let bottom = Vec2D::new(target_position.x, target_position.y + 1);

            if is_position_empty(top, elves)
                && is_position_empty(target_position, elves)
                && is_position_empty(bottom, elves)
            {
                return Some(target_position);
            }
        }

        if direction.y != 0 {
            let left = Vec2D::new(target_position.x - 1, target_position.y);
            let right = Vec2D::new(target_position.x + 1, target_position.y);

            if is_position_empty(left, elves)
                && is_position_empty(target_position, elves)
                && is_position_empty(right, elves)
            {
                return Some(target_position);
            }
        }
    }
    None
}

fn are_surroundings_empty(elves: &[Vec2D], point: Vec2D) -> bool {
    for direction in navigation::get_all_surrounding_directions() {
        if !is_position_empty(point + direction, elves) {
            return false;
        }
    }

    true
}

fn is_position_empty(point: Vec2D, elves: &[Vec2D]) -> bool {
    if elves.contains(&point) {
        return false;
    }

    true
}

fn get_directions() -> VecDeque<Vec2D> {
    VecDeque::from([
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 110);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), 20);
    }
}
