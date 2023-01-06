use aoc_helper::Runner;
mod test;

fn main() {
    let processor = Runner::from_input_file(parse);
    processor.process_and_write_part_1(part_1);
    processor.process_and_write_part_2(part_2);
}

fn parse(input: String) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .split(|line| line.is_empty())
        .map(|elf| {
            elf.iter()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}

fn part_1(calories: &Vec<u32>) -> u32 {
    *calories.iter().max().expect("no max found")
}

fn part_2(calories: &Vec<u32>) -> u32 {
    let mut calories = calories.to_vec();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum::<u32>()
}