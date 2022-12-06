fn main() {
    let mut calories = parse_input("input.txt");

    println!("answer 1: {}", part_1(&calories));
    println!("answer 2: {}", part_2(&mut calories));
}

fn parse_input(filename: &str) -> Vec<u32> {
    let input: String = std::fs::read_to_string(filename).expect("could not read input");
    let lines: Vec<&str> = input.lines().collect();
    lines
        .split(|line| line.is_empty())
        .map(|elf| elf.iter().map(|calories| calories.parse::<u32>().unwrap()).sum::<u32>())
        .collect()
}

fn part_1(calories: &Vec<u32>) -> u32{
    *calories.iter().max().expect("no max found")
}

fn part_2(calories: &mut Vec<u32>) -> u32{
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(&parse_input("test.txt")), 24000);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(&mut parse_input("test.txt")), 45000);
    }
}
