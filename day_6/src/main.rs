use itertools::Itertools;

fn main() {
    let chars = read_input("input.txt");

    println!("part 1: {}", part_1(&chars));
    println!("part 2: {}", part_2(&chars));
}

fn part_1(chars: &[char]) -> usize {
    get_index_of_first_contiguous_unique_subset(chars, 4)
}

fn part_2(chars: &[char]) -> usize {
    get_index_of_first_contiguous_unique_subset(chars, 14)
}

fn get_index_of_first_contiguous_unique_subset(chars: &[char], length: usize) -> usize {
    let (n, _) = chars[..]
        .windows(length)
        .enumerate()
        .find(|(_, window)| window.len() == window.iter().unique().count())
        .expect("No start token found");
    n + length
}

fn read_input(filename: &str) -> Vec<char> {
    let input = std::fs::read_to_string(filename).expect("Could not read file");
    input.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let n = part_1(&read_input("test.txt"));
        assert_eq!(n, 7);
    }

    #[test]
    fn part_2_works() {
        let n = part_2(&read_input("test.txt"));
        assert_eq!(n, 19);
    }
}
