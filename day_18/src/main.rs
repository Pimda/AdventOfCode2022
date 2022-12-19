use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

type Point = (i32, i32, i32);

fn main() {
    let cubes = read_input("input.txt");

    println!("Part 1: {}", part_1(&cubes));
    println!("Part 2: {}", part_2(&cubes));
}

fn part_1(cubes: &HashMap<Point, Cube>) -> i32 {
    let mut total_surface = 0;
    let neighbors = get_directions();

    for cube in cubes.values() {
        for neighbor in &neighbors {
            let key = add_tuples(&cube.as_tuple(), neighbor);

            if cubes.get(&key).is_none() {
                total_surface += 1;
            }
        }
    }

    total_surface
}

fn part_2(cubes: &HashMap<Point, Cube>) -> i32 {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = get_bounds(cubes);

    let width: usize = (max_x - min_x).try_into().unwrap();
    let height: usize = (max_y - min_y).try_into().unwrap();
    let depth: usize = (max_z - min_z).try_into().unwrap();

    let mut space = vec![vec![vec![Type::AirPocket; width + 1]; height + 1]; depth + 1];

    for cube in cubes.values() {
        let z: usize = (cube.z - min_z).try_into().unwrap();
        let y: usize = (cube.y - min_y).try_into().unwrap();
        let x: usize = (cube.x - min_x).try_into().unwrap();
        space[z][y][x] = Type::Block;
    }

    // flood fill starting from all edge blocks, to ensure no air pocket is missed
    for y in 0..=height {
        for z in 0..=depth {
            flood_fill(
                &mut space,
                (0 as i32, y as i32, z as i32),
                width,
                height,
                depth,
            );
            flood_fill(
                &mut space,
                (width as i32, y as i32, z as i32),
                width,
                height,
                depth,
            );
        }
    }

    for x in 0..=width {
        for z in 0..=depth {
            flood_fill(
                &mut space,
                (x as i32, 0 as i32, z as i32),
                width,
                height,
                depth,
            );
            flood_fill(
                &mut space,
                (x as i32, height as i32, z as i32),
                width,
                height,
                depth,
            );
        }
    }

    for x in 0..=width {
        for y in 0..=height {
            flood_fill(
                &mut space,
                (x as i32, y as i32, 0 as i32),
                width,
                height,
                depth,
            );
            flood_fill(
                &mut space,
                (x as i32, y as i32, depth as i32),
                width,
                height,
                depth,
            );
        }
    }

    let mut total_surface = 0;
    let neighbors = get_directions();

    // for z in 0..=depth {
    //     for x in 0..=(width + 2) {
    //         print!("-");
    //     }
    //     println!();
    //     for y in 0..=height {
    //         print!("|");
    //         for x in 0..=width {
    //             match space[z][y][x] {
    //                 Type::AirPocket => print!("."),
    //                 Type::Block => print!("#"),
    //                 Type::Outside => print!(" "),
    //             }
    //         }
    //         println!("|");
    //     }

    //     //stdin().read_line(&mut "".to_owned());
    // }

    for cube in cubes.values() {
        for neighbor in &neighbors {
            let key = add_tuples(&cube.as_tuple(), neighbor);

            let z = key.2 - min_z;
            let y = key.1 - min_y;
            let x = key.0 - min_x;

            if z < 0 || z > depth as i32 || y < 0 || y > height as i32 || x < 0 || x > width as i32
            {
                total_surface += 1;
                continue;
            }

            if space[z as usize][y as usize][x as usize] == Type::Outside {
                total_surface += 1;
            }
        }
    }

    total_surface
}

fn flood_fill(
    space: &mut Vec<Vec<Vec<Type>>>,
    start: Point,
    width: usize,
    height: usize,
    depth: usize,
) {
    let mut queue = VecDeque::new();

    let z: usize = start.2.try_into().unwrap();
    let y: usize = start.1.try_into().unwrap();
    let x: usize = start.0.try_into().unwrap();

    if space[z][y][x] == Type::Block {
        return;
    }

    space[z][y][x] = Type::Outside;

    queue.push_back(start);

    let directions = get_directions();

    loop {
        let point = queue.pop_front();

        if let Some(point) = point {
            for direction in &directions {
                let new_point = add_tuples(&point, direction);

                if new_point.0 < 0
                    || new_point.1 < 0
                    || new_point.2 < 0
                    || new_point.0 >= width as i32
                    || new_point.1 >= height as i32
                    || new_point.2 >= depth as i32
                {
                    continue;
                }

                let z: usize = new_point.2.try_into().unwrap();
                let y: usize = new_point.1.try_into().unwrap();
                let x: usize = new_point.0.try_into().unwrap();

                if space[z][y][x] == Type::AirPocket {
                    space[z][y][x] = Type::Outside;
                    queue.push_back((x as i32, y as i32, z as i32));
                }
            }
        } else {
            break;
        }
    }
}

fn get_directions() -> Vec<(i32, i32, i32)> {
    let neighbors = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    neighbors
}

fn get_bounds(cubes: &HashMap<(i32, i32, i32), Cube>) -> (i32, i32, i32, i32, i32, i32) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();
    let mut min_z = i32::max_value();
    let mut max_z = i32::min_value();

    for cube in cubes.values() {
        if cube.x < min_x {
            min_x = cube.x;
        }
        if cube.x > max_x {
            max_x = cube.x;
        }
        if cube.y < min_y {
            min_y = cube.y;
        }
        if cube.y > max_y {
            max_y = cube.y;
        }
        if cube.z < min_z {
            min_z = cube.z;
        }
        if cube.z > max_z {
            max_z = cube.z;
        }
    }
    (min_x, max_x, min_y, max_y, min_z, max_z)
}

fn add_tuples(lhs: &Point, rhs: &Point) -> Point {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}

fn read_input(filename: &str) -> HashMap<Point, Cube> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    let all_cubes: Vec<Cube> = string.lines().map(Cube::from_string).collect();
    let mut hash_map = HashMap::new();

    for cube in all_cubes {
        hash_map.insert(cube.as_tuple(), cube);
    }

    hash_map
}

struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn from_string(string: &str) -> Self {
        let mut chars = string.split(',');
        Self {
            x: chars.next().unwrap().parse().unwrap(),
            y: chars.next().unwrap().parse().unwrap(),
            z: chars.next().unwrap().parse().unwrap(),
        }
    }

    fn as_tuple(&self) -> Point {
        (self.x, self.y, self.z)
    }
}

#[derive(Clone, PartialEq)]
enum Type {
    AirPocket,
    Block,
    Outside,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 64);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), 58);
    }
}
