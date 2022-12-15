use std::{
    fmt::Display,
    io::stdin,
    ops::{Add, Sub},
};

fn main() {
    let input = read_input("input.txt");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Vec<Point>> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    string
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(Point::from_string)
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn part_1(lines: &Vec<Vec<Point>>) -> u32 {
    let (lower_bound_x, upper_bound_x, lower_bound_y, upper_bound_y) = get_bounds(lines);

    let mut map = generate_map(
        upper_bound_x,
        lower_bound_x,
        upper_bound_y,
        lower_bound_y,
        lines,
    );

    let mut number_of_grains = 0;
    let mut current_coordinate = Point { x: 500, y: 0 };

    let down = Point { x: 0, y: 1 };
    let left = Point { x: -1, y: 1 };
    let right = Point { x: 1, y: 1 };

    loop {
        let down = &current_coordinate + &down;

        if down.y > upper_bound_y {
            return number_of_grains;
        }

        if get_element_from_map(&map, &down, &lower_bound_x, &lower_bound_y) == &Tile::Empty {
            current_coordinate = down;
            continue;
        }

        let left = &current_coordinate + &left;

        if left.x < lower_bound_x {
            return number_of_grains;
        }

        if get_element_from_map(&map, &left, &lower_bound_x, &lower_bound_y) == &Tile::Empty {
            current_coordinate = left;
            continue;
        }

        let right = &current_coordinate + &right;

        if right.x >= upper_bound_x {
            return number_of_grains;
        }

        if get_element_from_map(&map, &right, &lower_bound_x, &lower_bound_y) == &Tile::Empty {
            current_coordinate = right;
            continue;
        }

        set_element_on_map(
            &mut map,
            &current_coordinate,
            &lower_bound_x,
            &lower_bound_y,
            Tile::Sand,
        );

        number_of_grains += 1;
        current_coordinate = Point { x: 500, y: 0 };
    }
}

fn part_2(lines: &Vec<Vec<Point>>) -> u32 {
    let (lower_bound_x, upper_bound_x, lower_bound_y, upper_bound_y) = get_bounds(lines);

    let height = upper_bound_y - lower_bound_y;

    let lower_bound_x = lower_bound_x - height;
    let upper_bound_y = upper_bound_y + 1;

    let mut map = generate_map(
        upper_bound_x + height,
        lower_bound_x,
        upper_bound_y,
        lower_bound_y,
        lines,
    );

    let mut number_of_grains = 0;
    let mut current_coordinate = Point { x: 500, y: 0 };

    let down = Point { x: 0, y: 1 };
    let left = Point { x: -1, y: 1 };
    let right = Point { x: 1, y: 1 };

    loop {
        let down = &current_coordinate + &down;

        if down.y > upper_bound_y {
            place_grain(
                &mut map,
                &mut current_coordinate,
                lower_bound_x,
                lower_bound_y,
                &mut number_of_grains,
            );
            //draw_map(&map);
            continue;
        }

        if get_element_from_map(&map, &down, &lower_bound_x, &lower_bound_y) == &Tile::Empty {
            current_coordinate = down;
            continue;
        }

        let left = &current_coordinate + &left;

        if get_element_from_map(&map, &left, &lower_bound_x, &lower_bound_y) == &Tile::Empty {
            current_coordinate = left;
            continue;
        }

        let right = &current_coordinate + &right;

        if get_element_from_map(&map, &right, &lower_bound_x, &lower_bound_y) == &Tile::Empty {
            current_coordinate = right;
            continue;
        }

        place_grain(
            &mut map,
            &mut current_coordinate,
            lower_bound_x,
            lower_bound_y,
            &mut number_of_grains,
        );
        //draw_map(&map);

        if get_element_from_map(&map, &current_coordinate, &lower_bound_x, &lower_bound_y) == &Tile::Sand{
            return number_of_grains
        }
    }
}

fn place_grain(
    map: &mut Vec<Vec<Tile>>,
    current_coordinate: &mut Point,
    lower_bound_x: i32,
    lower_bound_y: i32,
    number_of_grains: &mut u32,
) {
    set_element_on_map(
        map,
        current_coordinate,
        &lower_bound_x,
        &lower_bound_y,
        Tile::Sand,
    );
    *number_of_grains += 1;
    *current_coordinate = Point { x: 500, y: 0 };
}

fn get_element_from_map<'a>(
    map: &'a [Vec<Tile>],
    coordinate: &Point,
    x_offset: &i32,
    y_offset: &i32,
) -> &'a Tile {
    let y: usize = (coordinate.y - y_offset).try_into().unwrap();
    let x: usize = (coordinate.x - x_offset).try_into().unwrap();
    &map[y][x]
}

fn set_element_on_map(
    map: &mut [Vec<Tile>],
    coordinate: &Point,
    x_offset: &i32,
    y_offset: &i32,
    tile: Tile,
) {
    let y: usize = (coordinate.y - y_offset).try_into().unwrap();
    let x: usize = (coordinate.x - x_offset).try_into().unwrap();
    map[y][x] = tile
}

fn draw_map(map: &Vec<Vec<Tile>>) {
    print!("\x1B[2J\x1B[1;1H");
    for line in map {
        for tile in line {
            match tile {
                Tile::Empty => print!(" "),
                Tile::Rock => print!("#"),
                Tile::Sand => print!("O"),
            }
        }
        println!();
    }
    let mut buff = "".to_owned();
    _ = stdin().read_line(&mut buff);
}

fn generate_map(
    upper_bound_x: i32,
    lower_bound_x: i32,
    upper_bound_y: i32,
    lower_bound_y: i32,
    lines: &Vec<Vec<Point>>,
) -> Vec<Vec<Tile>> {
    let width = (upper_bound_x - lower_bound_x + 1).try_into().unwrap();
    let height = (upper_bound_y - lower_bound_y + 1).try_into().unwrap();

    let mut map = vec![vec![Tile::Empty; width]; height];

    for line in lines {
        let mut line_iter = line.iter();
        let mut from_coordinate = line_iter.next().unwrap();

        for to_coordinate in line_iter {
            let segment_direction = (to_coordinate - from_coordinate).signum();

            let mut current_coordinate = from_coordinate;
            let mut next;

            while current_coordinate != to_coordinate {
                set_element_on_map(
                    &mut map,
                    current_coordinate,
                    &lower_bound_x,
                    &lower_bound_y,
                    Tile::Rock,
                );
                next = current_coordinate + &segment_direction;

                current_coordinate = &next;
            }

            set_element_on_map(
                &mut map,
                current_coordinate,
                &lower_bound_x,
                &lower_bound_y,
                Tile::Rock,
            );

            from_coordinate = to_coordinate;
        }
    }
    map
}

fn get_bounds(lines: &[Vec<Point>]) -> (i32, i32, i32, i32) {
    let lower_bound_x = lines
        .iter()
        .map(|l| l.iter().map(|p| p.x).min().unwrap())
        .min()
        .unwrap();
    let upper_bound_x = lines
        .iter()
        .map(|l| l.iter().map(|p| p.x).max().unwrap())
        .max()
        .unwrap();
    let lower_bound_y = 0;
    let upper_bound_y = lines
        .iter()
        .map(|l| l.iter().map(|p| p.y).max().unwrap())
        .max()
        .unwrap();
    (lower_bound_x, upper_bound_x, lower_bound_y, upper_bound_y)
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_string(string: &str) -> Self {
        if let [x, y] = string.split(',').collect::<Vec<&str>>()[..] {
            Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        } else {
            panic!("Could not parse")
        }
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let pairs = read_input("test.txt");
        assert_eq!(part_1(&pairs), 24);
    }

    #[test]
    fn part_2_works() {
        let pairs = read_input("test.txt");
        assert_eq!(part_2(&pairs), 93);
    }
}
