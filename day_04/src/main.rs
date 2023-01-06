use aoc_helper::{runner::{Runner, ProcessAndWrite}};
mod test;

fn main() {
    let runner = Runner::from_input_file(parse);
    runner.process_and_write_part_1(part_1);
    runner.process_and_write_part_2(part_2);
}

fn parse(input: String) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn part_1(input: &[String]) -> usize {
    input
        .iter()
        .map(split_string_twice)
        .map(parse_nested_tuple)
        .filter(has_full_overlap)
        .count()
}

fn part_2(input: &[String]) -> usize {
    input
        .iter()
        .map(split_string_twice)
        .map(parse_nested_tuple)
        .filter(has_partial_overlap)
        .count()
}

fn has_full_overlap(tuple: &(u32, u32, u32, u32)) -> bool {
    let (ll, lu, rl, ru) = tuple;
    ll <= rl && lu >= ru || rl <= ll && ru >= lu
}

fn has_partial_overlap(tuple: &(u32, u32, u32, u32)) -> bool {
    let (ll, lu, rl, ru) = tuple;
    ll <= rl && lu >= rl || ll <= ru && lu >= ru || ll < rl && lu > ru || rl < ll && ru > lu
}

fn parse_nested_tuple(tuple: ((&str, &str), (&str, &str))) -> (u32, u32, u32, u32) {
    let ll = tuple.0 .0.parse::<u32>().unwrap();
    let lu = tuple.0 .1.parse::<u32>().unwrap();
    let rl = tuple.1 .0.parse::<u32>().unwrap();
    let ru = tuple.1 .1.parse::<u32>().unwrap();
    (ll, lu, rl, ru)
}

fn split_string_twice(string: &String) -> ((&str, &str), (&str, &str)) {
    let (left, right) = split_string(string, ',');
    (split_string(left, '-'), split_string(right, '-'))
}

fn split_string(line: &str, char: char) -> (&str, &str) {
    if let [left, right] = line.split(char).collect::<Vec<&str>>()[..] {
        return (left, right);
    }
    panic!("Could not split string");
}