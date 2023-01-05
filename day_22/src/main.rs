extern crate aoc_helper;
use aoc_helper::{math, point2d::Point2D, upoint2d::UPoint2D};

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input.0, &input.1));
    println!("Part 2: {}", part_2(&input.0, &input.1, 50));
}

fn read_input(filename: &str) -> (Vec<String>, String) {
    let string = std::fs::read_to_string(filename).expect("File not found");
    let lines = string.lines().collect::<Vec<&str>>();
    let [map, moves] = lines[..].split(|l| l.is_empty()).collect::<Vec<&[&str]>>()[..] else {panic!("Input not valid")};
    let map: Vec<String> = map.iter().map(|s| s.to_owned().to_owned()).collect();
    let square_map = fill_map_to_square(map);

    (square_map, moves.first().unwrap().to_owned().to_owned())
}

fn fill_map_to_square(mut map: Vec<String>) -> Vec<String> {
    let width = map.iter().map(|l| l.len()).max().unwrap() + 1;
    let mut map = map
        .iter_mut()
        .map(|l| {
            if l.len() < width {
                *l += &" ".repeat(width - l.len());
            }
            l.to_owned()
        })
        .collect::<Vec<String>>();

    for _ in map.len()..width {
        map.push(" ".repeat(width))
    }

    map
}

fn part_1(map: &[String], moves: &str) -> usize {
    walk_path(map, moves, false, 0)
}

fn part_2(map: &[String], moves: &str, map_size: u32) -> usize {
    walk_path(map, moves, true, map_size)
}

fn walk_path(map: &[String], moves: &str, cube: bool, tile_size: u32) -> usize {
    let directions = get_directions();
    let map_size = get_map_size(map);
    let mut current_direction = 0i32;
    let start_x = find_first_available_start_x(map);
    let start_y = 0;
    let mut current_point = UPoint2D::new(start_x, start_y);
    let mut steps_string = "".to_owned();
    for (_, _move) in moves.chars().enumerate() {
        match _move {
            'R' => {
                walk(
                    &mut steps_string,
                    &mut current_point,
                    &directions,
                    &mut current_direction,
                    map,
                    &map_size,
                    cube,
                    tile_size,
                );

                current_direction = math::positive_mod(current_direction + 1, 4);
            }
            'L' => {
                walk(
                    &mut steps_string,
                    &mut current_point,
                    &directions,
                    &mut current_direction,
                    map,
                    &map_size,
                    cube,
                    tile_size,
                );
                current_direction = math::positive_mod(current_direction - 1, 4);
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
        &map_size,
        cube,
        tile_size,
    );

    calculate_score(current_point, current_direction)
}

fn calculate_score(current_point: UPoint2D, current_direction: i32) -> usize {
    1000 * (current_point.y + 1) + 4 * (current_point.x + 1) + current_direction as usize
}

fn find_first_available_start_x(map: &[String]) -> usize {
    map.first()
        .expect("map should always have a first row")
        .chars()
        .enumerate()
        .filter(|(_, char)| *char == '.')
        .next()
        .expect("no start position found")
        .0
}

fn get_map_size(map: &[String]) -> Point2D {
    Point2D::new(
        map.first().unwrap().chars().count() as i32,
        map.len() as i32,
    )
}

fn get_directions() -> Vec<Point2D> {
    vec![
        Point2D::new(1, 0),
        Point2D::new(0, 1),
        Point2D::new(-1, 0),
        Point2D::new(0, -1),
    ]
}

fn walk(
    steps_string: &mut String,
    current_point: &mut UPoint2D,
    directions: &[Point2D],
    current_direction: &mut i32,
    map: &[String],
    map_size: &Point2D,
    cube: bool,
    tile_size: u32,
) {
    if steps_string.is_empty() {
        return;
    }

    let steps: i32 = steps_string.parse().unwrap();
    for _ in 0..steps {
        let mut next_point =
            calculate_next_point(current_point, directions, current_direction, map_size);

        match map[next_point.y].chars().nth(next_point.x).unwrap() {
            ' ' => {
                if !find_wrappd_position(
                    &mut next_point,
                    directions,
                    current_direction,
                    map_size,
                    map,
                    current_point,
                    cube,
                    tile_size,
                ) {
                    break;
                }
            }
            '.' => {
                *current_point = next_point;
            }
            '#' => break,
            _ => panic!("unexpected value"),
        }
    }

    *steps_string = "".to_owned();
}

fn find_wrappd_position(
    next_point: &mut UPoint2D,
    directions: &[Point2D],
    current_direction: &mut i32,
    map_size: &Point2D,
    map: &[String],
    current_point: &mut UPoint2D,
    cube: bool,
    tile_size: u32,
) -> bool {
    loop {
        *next_point = calculate_next_point(next_point, directions, current_direction, map_size);

        match map[next_point.y].chars().nth(next_point.x).unwrap() {
            ' ' => {
                if cube {
                    return wrap_cube(map, current_point, current_direction, tile_size);
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
    current_point: &mut UPoint2D,
    current_direction: &mut i32,
    tile_size: u32,
) -> bool {
    let tile_index = *current_point / tile_size;
    let position_in_tile = (*current_point - tile_index * tile_size).as_upoint2d();

    let (new_tile_index, new_direction) = find_new_tile(tile_index, current_direction);

    let rotation = math::positive_mod(*current_direction - new_direction, 4);
    let offset = tile_size as i32 - 1;

    let mut offset_position = position_in_tile * 2 - offset;

    for _ in 0..rotation {
        offset_position.rotate_right();
    }

    let new_position_in_tile = (offset_position + offset) / 2;

    let mut x_offset = 0;
    let mut y_offset = 0;

    match new_direction {
        0 => x_offset = -offset,
        1 => y_offset = -offset,
        2 => x_offset = offset,
        3 => y_offset = offset,
        _ => panic!("invalid rotation"),
    }

    let new_position = (new_position_in_tile
        + new_tile_index * tile_size as i32
        + Point2D::new(x_offset, y_offset))
    .as_upoint2d();

    draw_map(
        map,
        current_point,
        current_direction,
        &new_position,
        new_direction,
    );

    if map[new_position.y].chars().nth(new_position.x).unwrap() == '#' {
        return false;
    }

    *current_point = new_position;
    *current_direction = new_direction;

    true
}

fn find_new_tile(tile_index: UPoint2D, current_direction: &mut i32) -> (Point2D, i32) {
    // note this mapping only works for input.txt
    let offset_map = vec![
        (UPoint2D::new(0, 2), 2, Point2D::new(1, 0), 0),
        (UPoint2D::new(0, 2), 3, Point2D::new(1, 1), 0),
        (UPoint2D::new(0, 3), 0, Point2D::new(1, 2), 3),
        (UPoint2D::new(0, 3), 1, Point2D::new(2, 0), 1),
        (UPoint2D::new(0, 3), 2, Point2D::new(1, 0), 1),
        (UPoint2D::new(1, 0), 2, Point2D::new(0, 2), 0),
        (UPoint2D::new(1, 0), 3, Point2D::new(0, 3), 0),
        (UPoint2D::new(1, 1), 2, Point2D::new(0, 2), 1),
        (UPoint2D::new(1, 1), 0, Point2D::new(2, 0), 3),
        (UPoint2D::new(1, 2), 1, Point2D::new(0, 3), 2),
        (UPoint2D::new(1, 2), 0, Point2D::new(2, 0), 2),
        (UPoint2D::new(2, 0), 3, Point2D::new(0, 3), 3),
        (UPoint2D::new(2, 0), 1, Point2D::new(1, 1), 2),
        (UPoint2D::new(2, 0), 0, Point2D::new(1, 2), 2),
    ];

    for (source, source_rotation, tile_index_to_test, target_rotation) in offset_map {
        if source == tile_index && source_rotation == *current_direction {
            return (tile_index_to_test, target_rotation);
        }
    }

    panic!("Mapping not found")
}

fn draw_map(
    map: &[String],
    current_point: &mut UPoint2D,
    current_direction: &mut i32,
    next_point: &UPoint2D,
    new_direction: i32,
) {
    for (y, line) in map.iter().enumerate() {
        if y % 8 != 0 && y != current_point.y && y != next_point.y {
            continue;
        }
        for (x, char) in line.chars().enumerate() {
            if x % 8 != 0 && x != current_point.x && x != next_point.x {
                continue;
            }
            if current_point.x == x && current_point.y == y {
                match *current_direction {
                    0 => print!(">"),
                    1 => print!("V"),
                    2 => print!("<"),
                    3 => print!("^"),
                    _ => panic!(),
                }
            } else if next_point.x == x && next_point.y == y {
                match new_direction {
                    0 => print!(">"),
                    1 => print!("V"),
                    2 => print!("<"),
                    3 => print!("^"),
                    _ => panic!(),
                }
            } else if char == '.' {
                print!(".");
            } else {
                print!("{}", char);
            }
        }
        println!();
    }
    //use std::io::stdin;
    //stdin().read_line(&mut "".to_string());
}

fn calculate_next_point(
    point: &UPoint2D,
    directions: &[Point2D],
    current_direction: &i32,
    map_size: &Point2D,
) -> UPoint2D {
    (*point + directions[*current_direction as usize])
        .positive_mod(map_size)
        .as_upoint2d()
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
