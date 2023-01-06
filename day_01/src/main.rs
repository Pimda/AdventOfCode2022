use aoc_helper::runner::{Runner, ProcessAndWrite};
mod test;

fn main() {
    let runner = Runner::from_input_file(parse);
    runner.process_and_write_part_1(part_1);
    runner.process_and_write_part_2(part_2);
}

fn parse(input: String) -> Vec<u32> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|elf| {
            elf.iter()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}

fn part_1(calories: &[u32]) -> u32 {
    *calories.iter().max().expect("no max found")
}

fn part_2(calories: &[u32]) -> u32 {
    let mut calories = calories.to_vec();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum::<u32>()
}
