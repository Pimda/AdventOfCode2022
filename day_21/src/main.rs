use core::panic;
use std::fmt::Display;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(input));
}

fn read_input(filename: &str) -> Vec<Monkey> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    string.lines().map(Monkey::from_string).collect()
}

fn part_1(monkeys: &[Monkey]) -> i64 {
    let Node::Leaf(Some(result)) = wrap_up_from_bottom(&build_tree(monkeys, "root")) else{panic!()};
    result
}

fn part_2(mut monkeys: Vec<Monkey>) -> i64 {
    // change the humn monkey to type Human
    monkeys.iter_mut().find(|m| m.id == "humn").unwrap()._type = MonkeyType::Human;

    let root = wrap_up_from_bottom(&build_tree(&monkeys, "root"));

    if let Node::Node(left, _, right) = root {
        if let (node @ Node::Node(_, _, _), Node::Leaf(Some(val)))
        | (Node::Leaf(Some(val)), node @ Node::Node(_, _, _)) = (*left, *right)
        {
            return wrap_up_from_top(node, val);
        }
    }
    panic!()
}

fn build_tree(monkeys: &[Monkey], key: &str) -> Node {
    let monkey = monkeys.iter().find(|m| m.id == key).unwrap();

    match &monkey._type {
        MonkeyType::Value(v) => Node::Leaf(*v),
        MonkeyType::Calculation(calculation) => Node::Node(
            Box::new(build_tree(monkeys, &calculation.lhs)),
            calculation.operator.clone(),
            Box::new(build_tree(monkeys, &calculation.rhs)),
        ),
        MonkeyType::Human => Node::Leaf(None),
    }
}

fn wrap_up_from_bottom(node: &Node) -> Node {
    match node {
        Node::Node(lhs, op, rhs) => match (wrap_up_from_bottom(lhs), wrap_up_from_bottom(rhs)) {
            (Node::Leaf(Some(left_val)), Node::Leaf(Some(right_val))) => {
                calculate_value_both_known(op, left_val, right_val)
            }
            (Node::Node(_, _, _), Node::Node(_, _, _)) => panic!(),
            (left, right) => Node::Node(Box::new(left), op.to_owned(), Box::new(right)),
        },
        Node::Leaf(v) => Node::Leaf(*v),
    }
}

fn calculate_value_both_known(op: &str, left_val: i64, right_val: i64) -> Node {
    Node::Leaf(match op {
        "+" => Some(left_val + right_val),
        "-" => Some(left_val - right_val),
        "/" => Some(left_val / right_val),
        "*" => Some(left_val * right_val),
        _ => panic!(),
    })
}

fn wrap_up_from_top(node: Node, target_value: i64) -> i64 {
    //node needs to be equal to value
    if let Node::Node(left, op, right) = node {
        match (*left, *right) {
            (Node::Leaf(Some(left_val)), node @ Node::Node(_, _, _)) => {
                let value = calculate_value_for_known_left(op, target_value, left_val);
                return wrap_up_from_top(node, value);
            }
            (node @ Node::Node(_, _, _), Node::Leaf(Some(right_val))) => {
                let value = calculate_value_for_known_right(op, target_value, right_val);
                return wrap_up_from_top(node, value);
            }
            (Node::Leaf(Some(left_val)), Node::Leaf(None)) => {
                return calculate_value_for_known_left(op, target_value, left_val)
            }
            (Node::Leaf(None), Node::Leaf(Some(right_val))) => {
                return calculate_value_for_known_right(op, target_value, right_val)
            }
            _ => panic!(),
        }
    }

    panic!()
}

fn calculate_value_for_known_right(op: String, target_value: i64, right_val: i64) -> i64 {
    match op.as_str() {
        "+" => target_value - right_val,
        "-" => target_value + right_val,
        "/" => target_value * right_val,
        "*" => target_value / right_val,
        _ => panic!(),
    }
}

fn calculate_value_for_known_left(op: String, target_value: i64, left_val: i64) -> i64 {
    match op.as_str() {
        "+" => target_value - left_val,
        "-" => left_val - target_value,
        "/" => left_val / target_value,
        "*" => target_value / left_val,
        _ => panic!(),
    }
}

struct Monkey {
    id: String,
    _type: MonkeyType,
}

impl Monkey {
    fn from_string(string: &str) -> Self {
        let [id, value] = string.split(": ").collect::<Vec<&str>>()[..] else {panic!()};

        Self {
            id: id.to_owned(),
            _type: MonkeyType::from_string(value),
        }
    }
}

enum MonkeyType {
    Value(Option<i64>),
    Calculation(Calculation),
    Human,
}

impl MonkeyType {
    fn from_string(string: &str) -> Self {
        match string.split(' ').collect::<Vec<&str>>()[..] {
            [val] => MonkeyType::Value(Some(val.parse().unwrap())),
            [lhs, op, rhs] => MonkeyType::Calculation(Calculation::from_parts(lhs, op, rhs)),
            _ => panic!(),
        }
    }
}

struct Calculation {
    lhs: String,
    rhs: String,
    operator: String,
}

impl Calculation {
    fn from_parts(lhs: &str, op: &str, rhs: &str) -> Self {
        Self {
            lhs: lhs.to_owned(),
            rhs: rhs.to_owned(),
            operator: op.to_owned(),
        }
    }
}

enum Node {
    Node(Box<Node>, String, Box<Node>),
    Leaf(Option<i64>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Node::Node(_, _, _) => "node".to_owned(),
                Node::Leaf(val) => {
                    match val {
                        Some(v) => v.to_string(),
                        None => "none".to_owned(),
                    }
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 152);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(nodes), 301);
    }
}
