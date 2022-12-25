fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn read_input(filename: &str) -> Vec<String> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    string.lines().map(|s| s.to_owned()).collect()
}

fn part_1(input: Vec<String>) -> String {
    write_number(input.into_iter().map(parse_number).sum())
}

fn parse_number(string: String) -> i64 {
    let mut val = 1;
    let mut result = 0;

    for char in string.chars().rev() {
        match char {
            '=' => result -= val * 2,
            '-' => result -= val,
            '1' => result += val,
            '2' => result += val * 2,
            _ => (),
        };

        val *= 5;
    }

    result
}

fn write_number(mut number: i64) -> String {
    let mut result = "".to_owned();
    
    while number > 0 {
        let digit = number % 5;

        match digit {
            0 => result.push('0'),
            1 => result.push('1'),
            2 => result.push('2'),
            3 => {
                result.push('=');
                number += 5
            }
            4 => {
                result.push('-');
                number += 5
            }
            _ => panic!(),
        }

        number /= 5;
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(input), "2=-1=0".to_owned());
    }
}
