use std::{collections::HashMap, vec};

fn main() {
    let input = read_input("input.txt");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Direction> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    string
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn part_1(directions: &Vec<Direction>) -> i64 {
    calculate(directions, 2022)
}

fn part_2(directions: &Vec<Direction>) -> i64 {
    calculate(directions, 1000000000000i64)
}


/// So a lot of experimentation had to happen to get this to work
/// My assumption of what is wrong is that the first detected cycle somehow starts wrong
/// This would probably be solved by checking the board state 
fn calculate(directions: &Vec<Direction>, mut total_blocks: i64) -> i64 {
    let max_blocks_to_try: i64 = (directions.len() * 5).try_into().unwrap();
    let simulation = simulate(&directions, max_blocks_to_try, true);

    println!(
        "Simulation 1: height: {}, blocks: {}, 'start height': {}, 'start blocks': {}",
        simulation.0, simulation.1, simulation.2 .0, simulation.2 .1
    );

    // these offsets are added after experimentation
    let start_blocks = simulation.2 .1 + 5;
    let start_height = simulation.2 .0 + 2;

    let blocks_per_cycle = simulation.1 - simulation.2 .1 - 5;
    let height_per_cycle = simulation.0 - simulation.2 .0 - 4;

    let test1 = simulate(&directions, start_blocks + 1 * blocks_per_cycle, false);
    let _test2 = simulate(&directions, start_blocks + 2 * blocks_per_cycle, false);
    let test3 = simulate(&directions, start_blocks + 3 * blocks_per_cycle, false);
    let test4 = simulate(&directions, start_blocks + 4 * blocks_per_cycle, false);
    let test5 = simulate(&directions, start_blocks + 5 * blocks_per_cycle, false);
    let test6 = simulate(&directions, start_blocks + 20 * blocks_per_cycle, false);

    assert_eq!(test5.0 - test4.0, test4.0 - test3.0);
    assert_eq!(test4.0 - start_height, 4 * height_per_cycle);
    assert_eq!(test5.0 - start_height, 5 * height_per_cycle);
    assert_eq!(test6.0 - start_height, 20 * height_per_cycle);

    total_blocks -= start_blocks;

    let known_cycle_count = total_blocks / blocks_per_cycle;

    println!("known cycles: {}", known_cycle_count);

    let unknown_blocks = total_blocks % blocks_per_cycle;

    println!("unknown blocks: {}", unknown_blocks);

    let simulation_3 = simulate(
        &directions,
        start_blocks + blocks_per_cycle + unknown_blocks,
        false,
    );

    println!("height of second run: {}", simulation_3.0);

    let unknown_height = simulation_3.0 - test1.0;

    let test7 = simulate(&directions, start_blocks + 20 * blocks_per_cycle + unknown_blocks, false);

    assert_eq!(test7.0, start_height + 20 * height_per_cycle + unknown_height);

    println!("height of unknown blocks: {}", unknown_height);

    start_height + known_cycle_count * height_per_cycle + unknown_height
}

fn simulate(
    directions: &Vec<Direction>,
    block_count: i64,
    break_on_cycle: bool,
) -> (i64, i64, (i64, i64)) {
    // create a field with worst-case height (only vertical bars stacked, with 3 extra for the start)
    let mut field = vec![vec![false; 7]; (4 * block_count + 3).try_into().unwrap()];

    let blocks = vec![
        BlockTypes::Horizontal,
        BlockTypes::Plus,
        BlockTypes::L,
        BlockTypes::Vertical,
        BlockTypes::Square,
    ];

    let mut blocks_iter = blocks.iter().enumerate().cycle();
    let mut directions_iter = directions.iter().enumerate().cycle();
    let mut seen_combinations = vec![];
    let mut height_and_count_at_combination = HashMap::new();
    let mut current_highest = 0;

    for _index in 0..block_count {
        let (block_index, block_type) = blocks_iter.next().unwrap();

        let start_y = current_highest + 3;
        let start_x = 2;

        let mut current_position = (start_x, start_y);

        let offsets = get_block_offsets(block_type);

        loop {
            // blow in the wind
            let (direction_index, direction) = directions_iter.next().unwrap();

            let direction_offset;
            match direction {
                Direction::Left => direction_offset = (-1, 0),
                Direction::Right => direction_offset = (1, 0),
            }
            try_move(&offsets, &mut current_position, direction_offset, &field);

            // fall
            let direction_offset = (0, -1);
            if !try_move(&offsets, &mut current_position, direction_offset, &field) {
                let positions = calculate_positions(&offsets, &current_position, (0, 0));
                let mut local_highest = 0;

                for position in positions {
                    let y: usize = position.1.try_into().unwrap();
                    let x: usize = position.0.try_into().unwrap();

                    if y as i64 + 1 > local_highest {
                        local_highest = y as i64 + 1;
                    }

                    if y as i64 + 1 > current_highest {
                        current_highest = y as i64 + 1;
                    }

                    field[y][x] = true;
                }

                let key = (block_index, direction_index);

                if seen_combinations.contains(&key) && break_on_cycle {
                    println!("cycle found at ({}, {}):", block_index, direction_index);
                    _print(&current_highest, &field);
                    return (local_highest, _index, height_and_count_at_combination[&key]);
                }
                height_and_count_at_combination.insert(key, (local_highest, _index));
                seen_combinations.push(key);

                break;
            }
        }
    }

    println!("all blocks simulated:");
    _print(&current_highest, &field);
    (current_highest, 0, (0, 0))
}

fn _print(current_highest: &i64, field: &Vec<Vec<bool>>) {
    //print field
    let skip = (current_highest - 10).max(0);
    for row in field.iter().skip(skip.try_into().unwrap()).take(13).rev() {
        print!("|");
        for field in row {
            if *field {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("|")
    }
    //stdin().read_line(&mut "".to_owned());
}

fn try_move(
    offsets: &Vec<(i64, i64)>,
    current_position: &mut (i64, i64),
    direction_offset: (i64, i64),
    field: &Vec<Vec<bool>>,
) -> bool {
    let new_positions = calculate_positions(offsets, &*current_position, direction_offset);
    if check_if_free(field, &new_positions) {
        *current_position = (
            current_position.0 + direction_offset.0,
            current_position.1 + direction_offset.1,
        );
        return true;
    }

    false
}

fn calculate_positions(
    offsets: &Vec<(i64, i64)>,
    current_position: &(i64, i64),
    direction_offset: (i64, i64),
) -> Vec<(i64, i64)> {
    let new_positions: Vec<(i64, i64)> = offsets
        .iter()
        .map(|o| {
            (
                current_position.0 + o.0 + direction_offset.0,
                current_position.1 + o.1 + direction_offset.1,
            )
        })
        .collect();
    new_positions
}

fn check_if_free(field: &Vec<Vec<bool>>, positions: &Vec<(i64, i64)>) -> bool {
    for position in positions {
        if position.0 < 0 || position.0 >= 7 || position.1 < 0 {
            return false;
        }

        let y: usize = position.1.try_into().unwrap();
        let x: usize = position.0.try_into().unwrap();

        if field[y][x] {
            return false;
        }
    }

    true
}

fn get_block_offsets(block_type: &BlockTypes) -> Vec<(i64, i64)> {
    match block_type {
        BlockTypes::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        BlockTypes::Plus => vec![(0, 1), (1, 1), (2, 1), (1, 2), (1, 0)],
        BlockTypes::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        BlockTypes::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        BlockTypes::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

enum BlockTypes {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 3068);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), 1514285714288);
    }
}
