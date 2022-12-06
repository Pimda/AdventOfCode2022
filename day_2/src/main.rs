fn main() {
    let moves = parse_input("input.txt");

    println!("answer 1: {}", calculate_part_1(&moves));
    println!("answer 2: {}", calculate_part_2(&moves));
}

fn parse_input(filename: &str) -> Vec<Vec<String>> {
    let input = std::fs::read_to_string(filename).expect("could not read file");
    input
        .lines()
        .map(|l| l.split(' ').map(|t| t.to_string()).collect())
        .collect()
}

fn calculate_part_1(moves: &[Vec<String>]) -> i32 {
    moves
        .iter()
        .map(|mov| {
            if let [a, b] = &mov[..] {
                let a = parse_left_input(a);
                let b = parse_right_input(b);

                let win = (((b - a) % 3) + 3) % 3;

                let score = match win {
                    0 => 3,
                    1 => 6,
                    2 => 0,
                    _ => panic!("Game state invalid"),
                };

                score + b
            } else {
                panic!()
            }
        })
        .sum()
}

fn calculate_part_2(moves: &[Vec<String>]) -> i32 {
    moves
        .iter()
        .map(|mov| {
            if let [a, b] = &mov[..] {
                let a = parse_left_input(a);
                let outcome = parse_outcome(b);

                let mut b = (a + outcome) % 3;

                if b == 0 {
                    b = 3;
                }

                let score = match outcome {
                    -1 => 0,
                    0 => 3,
                    1 => 6,
                    _ => panic!("Game state invalid"),
                };

                score + b
            } else {
                panic!()
            }
        })
        .sum()
}

fn parse_left_input(a: &str) -> i32 {
    match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Invalid input"),
    }
}

fn parse_right_input(b: &str) -> i32 {
    match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Invalid input"),
    }
}

fn parse_outcome(b: &str) -> i32 {
    match b {
        "X" => -1,
        "Y" => 0,
        "Z" => 1,
        _ => panic!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(calculate_part_1(&parse_input("test.txt")), 15);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(calculate_part_2(&parse_input("test.txt")), 12);
    }
}
