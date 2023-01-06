mod test;
use aoc_helper::{math, runner::{Runner, ProcessAndWrite}};

fn main() {
    let runner = Runner::from_input_file(parse);
    runner.process_and_write_part_1(part_1);
    runner.process_and_write_part_2(part_2);
}

fn parse(input: String) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|t| t.to_string()).collect())
        .collect()
}

fn part_1(moves: &[Vec<String>]) -> i32 {
    moves
        .iter()
        .map(|mov| {
            let [a, b] = &mov[..] else{panic!()};
            let a = parse_left_input(a);
            let b = parse_right_input(b);

            let win = math::positive_mod(b - a, 3);
            let score = (win + 1) % 3 * 3;

            score + b
        })
        .sum()
}

fn part_2(moves: &[Vec<String>]) -> i32 {
    moves
        .iter()
        .map(|mov| {
            let [a, b] = &mov[..] else{panic!()};
            {
                let a = parse_left_input(a);
                let outcome = parse_outcome(b);

                let mut b = (a + outcome) % 3;
                if b == 0 {
                    b = 3;
                }

                let score = (outcome + 1) * 3;

                score + b
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
