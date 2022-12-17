use std::fmt::Display;

fn main() {
    let sensors = read_input("input.txt");

    println!("part 1: {}", part_1(&sensors, 2000000));
    println!("part 2: {}", part_2(&sensors, 4000000));
}

fn part_1(sensors: &[Sensor], row: i32) -> u32 {
    let mut count = 0;

    // just choose some ridiculously large range, that should work
    for x in -10000000..10000000 {
        let point = Point::new(x, row);

        if !sensors.iter().all(|s| {
            s.position.manhattan_distance(&point) > s.distance || s.closest_beacon == point
        }) {
            count += 1;
        }
    }

    count
}

/// So, I realise I should check the edges of all the rhumbuses, and find where they all overlap
/// But since brute forcing with jumps seems kinda doable, it's fun to see if it really is
fn part_2(sensors: &[Sensor], range: i32) -> i32 {
    for y in 689000..=range {
        let mut x_iter = 0..=range;

        while let Some(x) = x_iter.next() {
            let point = Point::new(x, y);

            if sensors
                .iter()
                .all(|s| s.position.manhattan_distance(&point) > s.distance)
            {
                return x * 4000000 + y;
            }
        }

        if y % 1000 == 0 {
            println!("row {}, {}% finished", y, y as f32 * 100.0 / range as f32);
        }
    }

    0
}

fn read_input(filename: &str) -> Vec<Sensor> {
    //Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let string = std::fs::read_to_string(filename).expect("File not found");
    string
        .lines()
        .map(|l| {
            if let [sensor, beacon] = l.split(": ").collect::<Vec<&str>>()[..] {
                let sensor = sensor.trim_start_matches("Sensor at ");
                let beacon = beacon.trim_start_matches("closest beacon is at ");

                Sensor::new(Point::from_string(sensor), Point::from_string(beacon))
            } else {
                panic!("Input invalid")
            }
        })
        .collect()
}

struct Sensor {
    position: Point,
    closest_beacon: Point,
    distance: i32,
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.position, self.closest_beacon)
    }
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Self {
        let distance = position.manhattan_distance(&closest_beacon);
        Self {
            position,
            closest_beacon,
            distance,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn from_string(string: &str) -> Self {
        if let [x, y] = string.split(", ").collect::<Vec<&str>>()[..] {
            let x = x.trim_start_matches("x=");
            let y = y.trim_start_matches("y=");

            Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        } else {
            panic!("Input invalid")
        }
    }

    fn manhattan_distance(&self, rhs: &Self) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let pairs = read_input("test.txt");
        assert_eq!(part_1(&pairs, 10), 26);
    }

    #[test]
    fn part_2_works() {
        let pairs = read_input("test.txt");
        assert_eq!(part_2(&pairs, 20), 56000011);
    }
}
