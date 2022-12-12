fn main() {
    let mut monkeys = read_input("input.txt");
    println!("part 1: {}", part_1(&mut monkeys));

    let mut monkeys = read_input("input.txt");
    println!("part 2: {}", part_2(&mut monkeys));
}

fn read_input(filename: &str) -> Vec<Monkey> {
    let string = std::fs::read_to_string(filename).expect("File not found");

    let lines = string.lines().collect::<Vec<&str>>();
    let monkey_inputs = lines.split(|l| l.is_empty()).collect::<Vec<&[&str]>>();

    monkey_inputs.iter().map(|m| Monkey::new(m)).collect()
}

fn part_1(monkeys: &mut Vec<Monkey>) -> u64 {
    calc(monkeys, 20, true)
}

fn part_2(monkeys: &mut Vec<Monkey>) -> u64 {
    calc(monkeys, 10000, false)
}

fn calc(monkeys: &mut Vec<Monkey>, turns: usize, divide: bool) -> u64 {

    let modulus: u64 = monkeys.iter().map(|m| m.divisible_test).product();

    for _ in 0..turns {
        for i in 0..monkeys.len() {
            let active_monkey = &mut monkeys[i];

            let mut moved_items = vec![];

            for item in &active_monkey.items {

                let var_1 = parse_var(&active_monkey.operation[0], item);
                let var_2 = parse_var(&active_monkey.operation[2], item);

                let mut new_value = calculate_new_value(&active_monkey.operation[1], var_1 % modulus, var_2 % modulus);

                if divide{
                    new_value /= 3;
                }

                let throw_to_id = match new_value % active_monkey.divisible_test == 0 {
                    true => active_monkey.true_monkey_id,
                    false => active_monkey.false_monkey_id,
                };

                moved_items.push((throw_to_id, new_value));
            }

            active_monkey.inspection_count += active_monkey.items.len();
            active_monkey.items = vec![];

            for moved_item in moved_items {
                monkeys[moved_item.0].items.push(moved_item.1);
            }
        }
    }
    let mut scores = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<usize>>();
    scores.sort_unstable();
    if let [item1, item2] = scores.iter().rev().take(2).collect::<Vec<&usize>>()[..] {
        let item1: u64 = (*item1).try_into().unwrap();
        let item2: u64 = (*item2).try_into().unwrap();
        return item1 * item2;
    }
    panic!()
}

fn calculate_new_value(operation: &str, var_1: u64, var_2: u64) -> u64 {
    match operation {
        "+" => var_1 + var_2,
        "*" => var_1 * var_2,
        _ => panic!("Invalid operator"),
    }
}

fn parse_var(operation: &str, item: &u64) -> u64 {
    match operation {
        "old" => *item,
        num => num.parse().unwrap(),
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: [String; 3],
    divisible_test: u64,
    true_monkey_id: usize,
    false_monkey_id: usize,
    inspection_count: usize,
}

impl Monkey {
    fn new(lines: &[&str]) -> Self {
        /*
        Suff to parse:

        0:Monkey 0:
        1:  Starting items: 79, 98
        2:  Operation: new = old * 19
        3:  Test: divisible by 23
        4:    If true: throw to monkey 2
        5:    If false: throw to monkey 3
        */
        let items = lines[1]
            .trim_start_matches("  Starting items: ")
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let operation = &lines[2]
            .trim_start_matches("  Operation: new = ")
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()[..];

        let operation: &[String; 3] = operation.try_into().unwrap();
        let operation = operation.to_owned();

        let divisible_test = lines[3]
            .trim_start_matches("  Test: divisible by ")
            .parse()
            .unwrap();

        let true_monkey_id = lines[4]
            .trim_start_matches("    If true: throw to monkey ")
            .parse()
            .unwrap();

        let false_monkey_id = lines[5]
            .trim_start_matches("    If false: throw to monkey ")
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            divisible_test,
            true_monkey_id,
            false_monkey_id,
            inspection_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let mut monkeys = read_input("test.txt");
        assert_eq!(part_1(&mut monkeys), 10605);
    }

    #[test]
    fn part_2_works() {
        let mut monkeys = read_input("test.txt");
        assert_eq!(part_2(&mut monkeys), 2713310158);
    }
}
