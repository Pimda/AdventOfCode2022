use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<(i32, i32)> {
    let string = std::fs::read_to_string(filename).expect("File not found");

    let mut elves = vec![];

    for (y, line) in string.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push((x as i32, y as i32));
            }
        }
    }

    elves
}

fn part_1(elves: &[(i32, i32)]) -> i32 {
    let elves = simulate(elves, 10).0;

    empty_spaces_in_smallest_rectangle(elves)
}

fn part_2(elves: &[(i32, i32)]) -> usize {
    simulate(elves, usize::max_value()).1
}

fn empty_spaces_in_smallest_rectangle(elves: Vec<(i32, i32)>) -> i32 {
    let min_x = elves.iter().map(|e| e.0).min().unwrap();
    let max_x = elves.iter().map(|e| e.0).max().unwrap();
    let min_y = elves.iter().map(|e| e.1).min().unwrap();
    let max_y = elves.iter().map(|e| e.1).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    width * height - elves.len() as i32
}

fn simulate(elves: &[(i32, i32)], max_turns: usize) -> (Vec<(i32, i32)>, usize) {
    let mut directions = get_directions();
    let mut elves = elves.to_vec();
    for i in 0..max_turns {
        let mut proposals = HashMap::new();

        for elf in elves.iter() {
            if are_surroundings_empty(&elves, elf) {
                proposals.insert(elf, (elf.0, elf.1));
                continue;
            }

            let target_position = get_first_free_position(elf, &directions, &elves);

            if let Some(new_position) = target_position {
                let new_position = (new_position.0, new_position.1);
                proposals.insert(elf, new_position);
                continue;
            }

            proposals.insert(elf, (elf.0, elf.1));
        }

        if proposals.iter().all(|(s, t)| *s == t){
            return (vec![], i + 1);
        }

        let mut new_positions = vec![];

        for (source, target) in proposals.iter() {
            if proposals.iter().filter(|(_, t)| *t == target).count() == 1 {
                new_positions.push(*target);
            } else {
                new_positions.push(**source);
            }
        }

        elves = new_positions;

        //print(height, width, &elves);

        let item = directions.pop_front().unwrap();
        directions.push_back(item);
    }
    (elves, 0)
}

fn _print(height: &i32, width: &i32, elves: &[(i32, i32)]) {
    for y in 0..*height {
        for x in 0..*width {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    stdin().read_line(&mut "".to_owned());
}

fn get_first_free_position(
    point: &(i32, i32),
    directions: &VecDeque<(i32, i32)>,
    elves: &[(i32, i32)],
) -> Option<(i32, i32)> {
    for direction in directions {
        let target_position = (point.0 + direction.0, point.1 + direction.1);

        if direction.0 != 0 {
            let top = (target_position.0, target_position.1 - 1);
            let bottom = (target_position.0, target_position.1 + 1);

            if is_position_empty(&top, elves)
                && is_position_empty(&target_position, elves)
                && is_position_empty(&bottom, elves)
            {
                return Some(target_position);
            }
        }

        if direction.1 != 0 {
            let left = (target_position.0 - 1, target_position.1);
            let right = (target_position.0 + 1, target_position.1);

            if is_position_empty(&left, elves)
                && is_position_empty(&target_position, elves)
                && is_position_empty(&right, elves)
            {
                return Some(target_position);
            }
        }
    }
    None
}

fn are_surroundings_empty(elves: &[(i32, i32)], point: &(i32, i32)) -> bool {
    let directions = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for direction in directions {
        let x = point.0 + direction.0;
        let y = point.1 + direction.1;

        if !is_position_empty(&(x, y), elves) {
            return false;
        }
    }

    true
}

fn is_position_empty(point: &(i32, i32), elves: &[(i32, i32)]) -> bool {
    if elves.contains(&(point.0, point.1)) {
        return false;
    }

    true
}

fn get_directions() -> VecDeque<(i32, i32)> {
    let mut directions = VecDeque::new();

    directions.push_back((0, -1));
    directions.push_back((0, 1));
    directions.push_back((-1, 0));
    directions.push_back((1, 0));

    directions
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
