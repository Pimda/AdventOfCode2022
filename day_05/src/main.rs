use aoc_helper::runner::{ProcessAndWrite, Runner};
mod test;

type Stack = Vec<char>;
type Step = (u32, usize, usize);
type Input = (Vec<Stack>, Vec<Step>);

fn main() {
    let runner = Runner::from_input_file(parse);
    runner.process_and_write_part_1(part_1);
    runner.process_and_write_part_2(part_2);
}

fn parse(string: String) -> Input {
    let lines = string
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();

    let [board, steps] = lines
        .split(|line| line.is_empty())
        .collect::<Vec<&[String]>>()[..]
    else {panic!()};

    let stacks = parse_containers(board);
    let steps = parse_steps(steps);

    (stacks, steps)
}

fn part_1(input: &Input) -> String {
    let (stacks, steps) = input;

    let mut stacks = stacks.to_vec();

    for step in steps {
        for _ in 0..step.0 {
            let container = stacks[step.1 - 1].pop().unwrap();
            stacks[step.2 - 1].push(container);
        }
    }

    row_to_string(get_top_row(stacks))
}

fn part_2(input: &Input) -> String {
    let (stacks, steps) = input;
    let mut stacks = stacks.to_vec();

    for step in steps {
        let mut temp: Vec<char> = vec![];
        for _ in 0..step.0 {
            let container = stacks[step.1 - 1].pop().unwrap();
            temp.push(container);
        }
        for _ in 0..step.0 {
            let container = temp.pop().unwrap();
            stacks[step.2 - 1].push(container);
        }
    }

    row_to_string(get_top_row(stacks))
}

fn parse_containers(board: &[String]) -> Vec<Stack> {
    let stacks_bottom_to_top: Vec<&String> = board.iter().rev().skip(1).collect();
    let containers_per_line_bottom_to_top: Vec<Stack> = stacks_bottom_to_top
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|container| container[1].to_owned())
                .collect::<Stack>()
        })
        .collect();
    let mut stacks: Vec<Stack> = vec![];
    let mut line_iter = containers_per_line_bottom_to_top.iter();
    let first_line = line_iter.next().unwrap();
    for container in first_line {
        stacks.push(vec![*container]);
    }
    for line in line_iter {
        for (i, container) in line.iter().enumerate() {
            if *container != ' ' {
                stacks[i].push(*container);
            }
        }
    }
    stacks
}

fn parse_steps(steps: &[String]) -> Vec<Step> {
    steps
        .iter()
        .map(|string| {
            let parts = &mut string.split(' ');
            (
                parts.nth(1).unwrap().parse().unwrap(),
                parts.nth(1).unwrap().parse().unwrap(),
                parts.nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn get_top_row(stacks: Vec<Stack>) -> Vec<char> {
    stacks
        .iter()
        .map(|stack| stack.last().expect("").to_owned())
        .collect()
}

fn row_to_string(top_row: Vec<char>) -> String {
    top_row.into_iter().collect()
}