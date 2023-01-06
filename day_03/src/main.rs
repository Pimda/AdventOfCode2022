use aoc_helper::{runner::{Runner, ProcessAndWrite}};
mod test;

fn main() {
    let runner = Runner::from_input_file(parse);
    runner.process_and_write_part_1(part_1);
    runner.process_and_write_part_2(part_2);
}

fn parse(input: String) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

fn part_1(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|line| {
            let bag_count = line.chars().count() / 2;

            let (left_compartment, right_compartment) = line.split_at(bag_count);
            let common_char =
                find_common_char(&[left_compartment.to_owned(), right_compartment.to_owned()]);
            get_value(common_char)
        })
        .sum()
}

fn part_2(lines: &[String]) -> i32 {
    lines.chunks(3).map(find_common_char).map(get_value).sum()
}

fn find_common_char(strings: &[String]) -> char {
    if let [leading, rest @ ..] = strings {
        return leading
            .chars()
            .find(|char| rest.iter().all(|string| string.contains(*char)))
            .expect("No common char found");
    }
    panic!("Not enough strings supplied");
}

/// Returns value of letter, where upper case is higher than lower case, one based.
fn get_value(char: char) -> i32 {
    let value = char as i32 - 'a' as i32;
    if value < 0 {
        return value + 58 + 1; // 58 is the diff between 'a' and 'A'
    }
    else {
        return value + 1
    }
}
