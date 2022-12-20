fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<i64> {
    let string = std::fs::read_to_string(filename).expect("File not found");

    string.lines().map(|s| s.parse().unwrap()).collect()
}

fn part_1(input: &[i64]) -> i64 {
    let mut result = create_result_vec(input);
    perform_cycle(input, &mut result);
    calculate_result(result)
}

fn part_2(input: &[i64]) -> i64 {
    let key = 811589153;
    let input: Vec<i64> = input.iter().map(|v| v * key).collect();

    let mut result = create_result_vec(&input);

    for _ in 0..10{
        perform_cycle(&input, &mut result);
    }

    calculate_result(result)
}

///result contains the original indexes and the corresponding values
fn create_result_vec(input: &[i64]) -> Vec<(usize, i64)> {
    input
        .iter()
        .enumerate()
        .map(|(key, value)| (key, *value))
        .collect()
}

fn perform_cycle(input: &[i64], result: &mut Vec<(usize, i64)>) {
    let size = input.len();

    for number in input.iter().enumerate() {
        let current_index = result.iter().position(|i| i.0 == number.0).unwrap();
        let moving_item = result.remove(current_index);
        let movement = moving_item.1;
        let mut new_index = (current_index as i64 + movement) % (size - 1) as i64;

        if new_index == 0 && movement < 0 {
            result.push(moving_item)
        } else {
            if new_index < 0 {
                new_index += (size - 1) as i64
            }

            result.insert(new_index as usize, moving_item);
        }
    }
}

fn calculate_result(result: Vec<(usize, i64)>) -> i64 {
    let start_index = result.iter().position(|v| v.1 == 0).unwrap();

    let res1 = result[(start_index + 1000) % result.len()].1;
    let res2 = result[(start_index + 2000) % result.len()].1;
    let res3 = result[(start_index + 3000) % result.len()].1;

    res1 + res2 + res3
}

fn _print_sequence(result: &Vec<(usize, i64)>) {
    for (_, n) in result {
        print!("{} ", n);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), 1623178306);
    }
}
