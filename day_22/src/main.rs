use std::io::stdin;

fn main() {
    let input = read_input("input.txt");

    //println!("Part 1: {}", part_1(&input.0, &input.1));
    println!("Part 2: {}", part_2(&input.0, &input.1, 50));
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
    let width = map.iter().map(|l| l.len()).max().unwrap() + 1;
    let mut map = map.iter_mut()
        .map(|l| {
            if l.len() < width {
                *l += &" ".repeat(width - l.len());
            }
            l.to_owned()
        })
        .collect::<Vec<String>>();

    for i in map.len()..width{
        map.push(" ".repeat(width))
    }

    map
}

fn part_1(map: &[String], moves: &str) -> usize {
    walk_path(map, moves, false, 0)
}

fn part_2(map: &[String], moves: &str, map_size: i32) -> usize {
    walk_path(map, moves, true, map_size)
}

fn walk_path(map: &[String], moves: &str, cube: bool, map_size: i32) -> usize {
    let directions = get_directions();
    let (width, height) = get_map_size(map);
    let mut current_direction = 0i32;
    let start_x = find_first_available_start_x(map);
    let start_y = 0;
    let mut current_point = (start_x, start_y);
    let mut steps_string = "".to_owned();
    for (move_index, _move) in moves.chars().enumerate() {
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
                    map_size,
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
                    map_size,
                );
                current_direction = (current_direction - 1) % 4
            }
            char => steps_string.push(char),
        }
    }

    walk(
        &mut steps_string,
        &mut current_point,
        &directions,
        &mut current_direction,
        map,
        width,
        height,
        cube,
        map_size,
    );

    calculate_score(current_point, current_direction)
}

fn calculate_score(current_point: (usize, usize), current_direction: i32) -> usize {
    println!("Position: ({}, {})", current_point.0 + 1, current_point.1 + 1);
    1000 * (current_point.1 + 1) + 4 * (current_point.0 + 1) + (current_direction % 4) as usize
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
    map_size: i32,
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
                    map_size,
                ) {
                    break;
                }
            }
            '.' => {
                *current_point = next_point;
                draw_map(map, current_point, current_direction, (1000, 1000), 0);
            }
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
    map_size: i32,
) -> bool {
    loop {
        *next_point =
            calculate_next_point(next_point, directions, current_direction, width, height);

        match map[next_point.1].chars().nth(next_point.0).unwrap() {
            ' ' => {
                if cube {
                    return wrap_cube(map, current_point, current_direction, map_size);
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
    map_size: i32,
) -> bool {
    let tile_index = (
        current_point.0 as i32 / map_size,
        current_point.1 as i32 / map_size,
    );
    let position_in_tile = (
        current_point.0 as i32 - tile_index.0 as i32 * map_size,
        current_point.1 as i32 - tile_index.1 as i32 * map_size,
    );

    println!(
        "I'm at tile index: ({},{}), facing {}",
        tile_index.0, tile_index.1, current_direction
    );

    let (new_tile_index, new_direction) =
        find_new_tile(map, map_size, tile_index, current_direction);

    let rotation = ((*current_direction - new_direction) % 4 + 4) % 4;
    let offset = map_size - 1;

    let mut offset_position = (
        position_in_tile.0 as i32 * 2 - offset,
        position_in_tile.1 as i32 * 2 - offset,
    );

    for _ in 0..rotation {
        offset_position = (offset_position.1, -offset_position.0);
    }

    let new_position_in_tile = (
        (offset_position.0 + offset) / 2,
        (offset_position.1 + offset) / 2,
    );

    let mut x_offset = 0;
    let mut y_offset = 0;

    let new_direction = (new_direction + 4) % 4;

    match new_direction {
        0 => x_offset = -offset,
        1 => y_offset = -offset,
        2 => x_offset = offset,
        3 => y_offset = offset,
        _ => panic!("invalid rotation"),
    }

    let new_position = (
        (new_position_in_tile.0 + new_tile_index.0 * map_size + x_offset) as usize,
        (new_position_in_tile.1 + new_tile_index.1 * map_size + y_offset) as usize,
    );

    draw_map(
        map,
        current_point,
        current_direction,
        new_position,
        new_direction,
    );

    if map[new_position.1].chars().nth(new_position.0).unwrap() == '#'{
        return false;
    }

    *current_point = new_position;
    *current_direction = new_direction;

    true
}

fn find_new_tile(
    map: &[String],
    map_size: i32,
    tile_index: (i32, i32),
    current_direction: &mut i32,
) -> ((i32, i32), i32) {
    let width_in_tiles = map[0].len() as i32 / map_size;
    let height_in_tiles = map.len() as i32 / map_size;

    *current_direction = (*current_direction + 4) % 4;

    let offset_map = vec![(-1, 3, 1, 100), (2, 1, 2, 100), (1, 1, 1, 0), (-2, -1, 2, 1), (1, -1, 1, 3)];

    for offset in offset_map {

        if offset.3 != *current_direction{
            continue;
        }

        let tile_index_to_test = (tile_index.0 + offset.0, tile_index.1 + offset.1);

        if tile_index_to_test.0 >= 0
            && tile_index_to_test.1 >= 0
            && tile_index_to_test.0 < width_in_tiles
            && tile_index_to_test.1 < height_in_tiles
        {
            // tile is within the map, now check if it is filled

            let position_to_check = (
                tile_index_to_test.0 * map_size,
                tile_index_to_test.1 * map_size,
            );
            
            if map[position_to_check.1 as usize]
                .chars()
                .nth(position_to_check.0 as usize)
                .unwrap()
                != ' '
            {
                // tile is filled
                return (tile_index_to_test, *current_direction + offset.2);
            }
        }
    }

    panic!("Mapping not found")
}

fn draw_map(
    map: &[String],
    current_point: &mut (usize, usize),
    current_direction: &mut i32,
    next_point: (usize, usize),
    new_direction: i32,
) {
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
                if char == '.' {
                    print!(".");
                } else {
                    print!("{}", char);
                }
            }
        }
        println!();
    }
    stdin().read_line(&mut "".to_string());
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
        assert_eq!(part_2(&input.0, &input.1, 4), 5031);
    }
}
