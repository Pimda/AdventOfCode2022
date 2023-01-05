use std::fs;

fn main() {
    let lines = get_lines_from_file("input.txt");

    println!("answer 1: {}", part1(&lines));
    println!("answer 2: {}", part2(&lines));
}

fn get_lines_from_file(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("File not found");
    let lines: Vec<String> = file.lines().map(|s| s.to_owned()).collect();
    lines
}

fn part1(lines: &[String]) -> i32 {
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

fn part2(lines: &[String]) -> i32 {
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

fn get_value(char: char) -> i32 {
    let mut value = char as i32 - 'a' as i32;
    if value < 0 {
        value += 58; // diff between 'a' and 'A'
    }
    value + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = get_lines_from_file("test.txt");
        assert_eq!(part1(&lines), 157);
    }

    #[test]
    fn test_part_2() {
        let lines = get_lines_from_file("test.txt");
        assert_eq!(part2(&lines), 70);
    }
}
