fn main() {
    let input = read_input("input.txt");

    println!("answer 1: {}", part_1(&input));
    println!("answer 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> i32 {
    let mut count = 0;
    for line in input {
        let (ll, lu, rl, ru) = parse_nested_tuple(split_string_twice(line, ',', '-'));

        if ll <= rl && lu >= ru || rl <= ll && ru >= lu {
            count += 1;
        }
    }
    count
}

fn part_2(input: &[String]) -> i32 {
    let mut count = 0;
    for line in input {
        let (ll, lu, rl, ru) = parse_nested_tuple(split_string_twice(line, ',', '-'));

        if ll <= rl && lu >= rl || ll <= ru && lu >= ru || ll < rl && lu > ru || rl < ll && ru > lu
        {
            count += 1;
        }
    }
    count
}

fn parse_nested_tuple(tuple: ((&str, &str), (&str, &str))) -> (u32, u32, u32, u32) {
    let ll = tuple.0 .0.parse::<u32>().expect("");
    let lu = tuple.0 .1.parse::<u32>().expect("");
    let rl = tuple.1 .0.parse::<u32>().expect("");
    let ru = tuple.1 .1.parse::<u32>().expect("");
    (ll, lu, rl, ru)
}

fn split_string_twice(
    string: &str,
    first_char: char,
    second_char: char,
) -> ((&str, &str), (&str, &str)) {
    let tuple = split_string(string, first_char);
    (
        split_string(tuple.0, second_char),
        split_string(tuple.1, second_char),
    )
}

fn split_string(line: &str, char: char) -> (&str, &str) {
    if let [left, right] = line.split(char).collect::<Vec<&str>>()[..] {
        return (left, right);
    }
    panic!("Could not split string");
}

fn read_input(filename: &str) -> Vec<String> {
    let content = std::fs::read_to_string(filename).expect("Could not find file");
    content.lines().map(|l| l.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("test.txt");
        assert_eq!(part_2(&input), 4);
    }
}
