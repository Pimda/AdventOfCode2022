use std::io::stdin;

fn main() {
    let input = read_input("input.txt");

    //println!("Part 1: {}", part_1(&input.0, &input.1));
    println!("Part 2: {}", part_2(&input.0, &input.1));
}

fn read_input(filename: &str) -> (Vec<String>, String) {
    let string = std::fs::read_to_string(filename).expect("File not found");
    let lines = string.lines().collect::<Vec<&str>>();
    let [map, moves] = lines[..].split(|l| l.is_empty()).collect::<Vec<&[&str]>>()[..] else {panic!()};
    let map: Vec<String> = map.iter().map(|s| s.to_owned().to_owned()).collect();

    let square_map = fill_map_to_square(map);

    (square_map, moves.first().unwrap().to_owned().to_owned())
}

fn fill_map_to_square(mut map: Vec<String>) -> Vec<String> {
    let width = map.first().unwrap().chars().count();
    map.iter_mut()
        .map(|l| {
            if l.len() < width {
                *l += &" ".repeat(width - l.len());
            }
            l.to_owned()
        })
        .collect()
}

fn part_1(map: &[String], moves: &str) -> usize {
    walk_path(map, moves, false)
}

fn part_2(map: &[String], moves: &str) -> usize {
    walk_path(map, moves, true)
}

fn walk_path(map: &[String], moves: &str, cube: bool) -> usize {
    let directions = get_directions();
    let (width, height) = get_map_size(map);
    let mut current_direction = 0i32;
    let start_x = find_first_available_start_x(map);
    let start_y = 0;
    let mut current_point = (start_x, start_y);
    let mut steps_string = "".to_owned();
    for (move_index, _move) in moves.chars().enumerate() {
        println!("Move: {}", move_index);

        match _move {
            'R' => {
                walk(
                    &mut steps_string,
                    &mut current_point,
                    &directions,
                    &mut current_direction,
                    map,
                    width,
                    height,
                    cube,
                );
                current_direction = (current_direction + 1) % 4
            }
            'L' => {
                walk(
                    &mut steps_string,
                    &mut current_point,
                    &directions,
                    &mut current_direction,
                    map,
                    width,
                    height,
                    cube,
                );
                current_direction = (current_direction - 1) % 4
            }
            char => steps_string.push(char),
        }
    }
    calculate_score(current_point, current_direction)
}

fn calculate_score(current_point: (usize, usize), current_direction: i32) -> usize {
    1000 * (current_point.1 + 1) + 4 * (current_point.0 + 1) + current_direction as usize
}

fn find_first_available_start_x(map: &[String]) -> usize {
    let mut x = 0;

    for (index, char) in map.first().unwrap().chars().enumerate() {
        if char == '.' {
            x = index;
            break;
        }
    }

    x
}

fn get_map_size(map: &[String]) -> (i32, i32) {
    let height = map.len() as i32;
    let width = map.first().unwrap().chars().count() as i32;
    (width, height)
}

fn get_directions() -> Vec<(i32, i32)> {
    vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
}

fn walk(
    steps_string: &mut String,
    current_point: &mut (usize, usize),
    directions: &[(i32, i32)],
    current_direction: &mut i32,
    map: &[String],
    width: i32,
    height: i32,
    cube: bool,
) {
    if steps_string.is_empty() {
        return;
    }

    let steps: i32 = steps_string.parse().unwrap();
    for _ in 0..steps {
        let mut next_point =
            calculate_next_point(current_point, directions, current_direction, width, height);

        match map[next_point.1].chars().nth(next_point.0).unwrap() {
            ' ' => {
                if !find_wrappd_position(
                    &mut next_point,
                    directions,
                    current_direction,
                    width,
                    height,
                    map,
                    current_point,
                    cube,
                ) {
                    break;
                }
            }
            '.' => *current_point = next_point,
            '#' => break,
            _ => panic!("unexpected value"),
        }
    }

    *steps_string = "".to_owned();
}

fn find_wrappd_position(
    next_point: &mut (usize, usize),
    directions: &[(i32, i32)],
    current_direction: &mut i32,
    width: i32,
    height: i32,
    map: &[String],
    current_point: &mut (usize, usize),
    cube: bool,
) -> bool {
    loop {
        *next_point =
            calculate_next_point(next_point, directions, current_direction, width, height);

        match map[next_point.1].chars().nth(next_point.0).unwrap() {
            ' ' => {
                if cube {
                    return wrap_cube(map, current_point, current_direction);
                }
            }
            '#' => return false,
            '.' => {
                *current_point = *next_point;
                break;
            }
            _ => panic!("unexpected value"),
        }
    }
    true
}

fn wrap_cube(
    map: &[String],
    current_point: &mut (usize, usize),
    current_direction: &mut i32,
) -> bool {
    *current_direction = (*current_direction % 4 + 4) % 4;

    println!(
        "I'm at ({},{}), heading: {}",
        current_point.0, current_point.1, current_direction
    );

    let next_point;
    let new_direction;

    if current_point.0 == 50
        && current_point.1 >= 0
        && current_point.1 < 50
        && *current_direction == 2
    {
        println!("block 1");
        next_point = (0, 50 - current_point.1 + 99);
        new_direction = 0;
    } else if current_point.1 == 0
        && current_point.0 >= 50
        && current_point.0 < 100
        && *current_direction == 3
    {
        println!("block 2");
        next_point = (0, current_point.0 - 50 + 150);
        new_direction = 0;
    } else if current_point.1 == 149 && current_point.0 >= 50 && current_point.0 < 100 {
        println!("block 3");
        next_point = (49, current_point.0 - 50 + 150);
        new_direction = 2;
    } else if current_point.0 == 0 && current_point.1 >= 100 && current_point.1 < 150 {
        println!("block 4");
        next_point = (50, 150 - current_point.1);
        new_direction = 0;
    } else if current_point.0 == 0 && current_point.1 >= 150 && current_point.1 < 200 {
        println!("block 5");
        next_point = (current_point.1 - 150 + 50, 0);
        new_direction = 1;
    } else if current_point.0 == 50 && current_point.1 >= 50 && current_point.1 < 100 {
        println!("block 6");
        next_point = (current_point.1 - 50, 100);
        new_direction = 1;
    } else if current_point.1 == 100 && current_point.0 >= 0 && current_point.0 < 50 {
        println!("block 7");
        next_point = (50, 50 + current_point.0);
        new_direction = 0;
    } else if current_point.0 == 99 && current_point.1 >= 100 && current_point.1 < 150 {
        println!("block 8");
        next_point = (149, 50 - (current_point.1 - 100));
        new_direction = 2;
    } else if current_point.1 == 49 && current_point.0 >= 100 && current_point.0 < 150 {
        println!("block 9");
        next_point = (99, current_point.0 - 50);
        new_direction = 2;
    } else if current_point.0 == 99 && current_point.1 >= 50 && current_point.1 < 100 {
        println!("block 10");
        next_point = (current_point.1 + 50, 49);
        new_direction = 3;
    } else if current_point.0 == 149 && current_point.1 >= 0 && current_point.1 < 50 {
        println!("block 11");
        next_point = (99, 150 - current_point.1);
        new_direction = 2;
    } else if current_point.1 == 0 && current_point.0 >= 100 && current_point.0 < 150 {
        println!("block 12");
        next_point = (current_point.0 - 100, 199);
        new_direction = 3;
    } else if current_point.0 == 50 && current_point.1 >= 0 && current_point.1 < 50 {
        println!("block 13");
        next_point = (0, 150 - current_point.1);
        new_direction = 0;
    } else if current_point.0 == 49 && current_point.1 >= 150 && current_point.1 < 200 {
        println!("block 14");
        next_point = (current_point.1 - 100, 149);
        new_direction = 3;
    } else if current_point.1 == 199 && current_point.0 >= 0 && current_point.0 < 50 {
        println!("block 15");
        next_point = (current_point.0 + 100, 0);
        new_direction = 1;
    } else {
        panic!("no mapping for position");
    }

    match map[next_point.1].chars().nth(next_point.0).unwrap() {
        '#' => return false,
        '.' => {
            for (y, line) in map.iter().enumerate() {
                for (x, char) in line.chars().enumerate() {
                    if current_point.0 == x && current_point.1 == y {
                        match ((*current_direction % 4) + 4) % 4 {
                            0 => print!(">"),
                            1 => print!("V"),
                            2 => print!("<"),
                            3 => print!("^"),
                            _ => panic!(),
                        }
                    } else if next_point.0 == x && next_point.1 == y {
                        match ((new_direction % 4) + 4) % 4 {
                            0 => print!(">"),
                            1 => print!("V"),
                            2 => print!("<"),
                            3 => print!("^"),
                            _ => panic!(),
                        }
                    } else {
                        if char == '.'{
                            print!(" ");
                        }
                        else{
                            print!("{}", char);
                        }
                    }
                }
                println!();
            }

            stdin().read_line(&mut "".to_owned());

            *current_point = next_point;
            *current_direction = new_direction;

            println!(
                "Continuing at position: ({},{}), with heading: {}",
                current_point.0, current_point.1, current_direction
            );

            return true;
        }
        _ => panic!("unexpected value"),
    }
}

fn calculate_next_point(
    point: &mut (usize, usize),
    directions: &[(i32, i32)],
    current_direction: &i32,
    width: i32,
    height: i32,
) -> (usize, usize) {
    let direction_index = ((current_direction + 4) % 4) as usize;
    let x = (width + point.0 as i32 + directions[direction_index].0) % width;
    let y = (height + point.1 as i32 + directions[direction_index].1) % height;

    (x as usize, y as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input.0, &input.1), 6032);
    }

    #[test]
    fn part_2_works() {
        let input = read_input("test.txt");
        assert_eq!(part_2(&input.0, &input.1), 117054);
    }
}
