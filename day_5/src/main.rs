type Stack = Vec<char>;
type Step = (u32, usize, usize);

fn main() {
    let lines = read_input("input.txt");

    let (containers, steps) = parse_input(lines);

    let top_row = part_1(&containers, &steps);

    println!("part 1");
    print_top_row(top_row);

    let top_row = part_2(&containers, &steps);

    println!("part 2");
    print_top_row(top_row);
}

fn print_top_row(top_row: Vec<char>) {
    for container in top_row {
        print!("{}", container);
    }
    println!();
}

fn part_1(stacks: &[Stack], steps: &[Step]) -> Vec<char> {
    let mut stacks = stacks.to_vec();

    for step in steps {
        for _ in 0..step.0 {
            let container = stacks[step.1 - 1].pop().unwrap();
            stacks[step.2 - 1].push(container);
        }
    }

    get_top_row(stacks)
}

fn part_2(stacks: &[Stack], steps: &[Step]) -> Vec<char> {
    let mut stacks = stacks.to_vec();

    for step in steps {
        let mut temp: Vec<char> = vec![];
        for _ in 0..step.0 {
            let container = stacks[step.1 - 1].pop().unwrap();
            temp.push(container);
        }
        for _ in 0..step.0 {
            let container = temp.pop().unwrap();
            stacks[step.2 - 1].push(container);
        }
    }

    get_top_row(stacks)
}

fn get_top_row(stacks: Vec<Stack>) -> Vec<char> {
    stacks
        .iter()
        .map(|stack| stack.last().expect("").to_owned())
        .collect()
}

fn read_input(filename: &str) -> Vec<String> {
    let content = std::fs::read_to_string(filename).expect("Could not find file");
    content.lines().map(|l| l.to_owned()).collect()
}

fn parse_input(lines: Vec<String>) -> (Vec<Stack>, Vec<Step>) {
    if let [board, steps] = lines
        .split(|line| line.is_empty())
        .collect::<Vec<&[String]>>()[..]
    {
        let stacks = parse_containers(board);
        let steps = parse_steps(steps);

        (stacks, steps)
    } else {
        panic!()
    }
}

fn parse_steps(steps: &[String]) -> Vec<Step> {
    steps
        .iter()
        .map(|string| {
            let parts = &mut string.split(' ');
            (
                parts.nth(1).unwrap().parse().unwrap(),
                parts.nth(1).unwrap().parse().unwrap(),
                parts.nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn parse_containers(board: &[String]) -> Vec<Stack> {
    let stacks_bottom_to_top: Vec<&String> = board.iter().rev().skip(1).collect();
    let containers_per_line_bottom_to_top: Vec<Stack> = stacks_bottom_to_top
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|container| container[1].to_owned())
                .collect::<Stack>()
        })
        .collect();
    let mut stacks: Vec<Stack> = vec![];
    let mut line_iter = containers_per_line_bottom_to_top.iter();
    let first_line = line_iter.next().unwrap();
    for container in first_line {
        stacks.push(vec![*container]);
    }
    for line in line_iter {
        for (i, container) in line.iter().enumerate() {
            if *container != ' ' {
                stacks[i].push(*container);
            }
        }
    }
    stacks
}

fn _print_steps(steps: &[Step]) {
    for step in steps {
        println!("move {} from {} to {}", step.0, step.1, step.2);
    }
}

fn _print_containers(containers: &[Stack]) {
    for line in containers {
        for container in line {
            print!("[{}] ", container);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = read_input("test.txt");
        let (containers, steps) = parse_input(lines);
        assert_eq!(part_1(&containers, &steps), vec!['C', 'M', 'Z']);
    }

    #[test]
    fn test_part_2() {
        let lines = read_input("test.txt");
        let (containers, steps) = parse_input(lines);
        assert_eq!(part_2(&containers, &steps), vec!['M', 'C', 'D']);
    }
}
