fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Monkey> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    string.lines().map(|m| Monkey::from_string(m)).collect()
}

fn part_1(monkeys: &Vec<Monkey> ) -> i64 {

    calculate_tree(&build_tree(monkeys, "root"))
}

fn part_2(monkeys: &Vec<Monkey> ) -> i64 {
    0
}

fn build_tree(monkeys: &Vec<Monkey>, key: &str) -> Node{

    let monkey = monkeys.iter().find(|m| m.id == key).unwrap();

    match &monkey._type{
        MonkeyType::Value(v) => Node::Leaf(*v),
        MonkeyType::Calculation(calculation) => {
            Node::Node(Box::new(build_tree(monkeys, &calculation.lhs)), calculation.operator.clone(), Box::new(build_tree(monkeys, &calculation.rhs)))
        },
    }
}

fn calculate_tree(node: &Node) -> i64{
    match node{
        Node::Node(lhs, op, rhs) => {
            let left = calculate_tree(lhs);
            let right = calculate_tree(rhs);

            match op.as_str(){
                "+" => left + right,
                "-" => left - right,
                "/" => left / right,
                "*" => left * right,
                _ => panic!()
            }
        },
        Node::Leaf(v) => *v,
    }
}

struct Monkey {
    id: String,
    _type: MonkeyType,
}

impl Monkey {
    fn from_string(string: &str) -> Self {
        let [id, value] = string.split(": ").collect::<Vec<&str>>()[..] else {panic!()};

        Self { id: id.to_owned(), _type: MonkeyType::from_string(value) }
    }
}

enum MonkeyType {
    Value(i64),
    Calculation(Calculation),
}

impl MonkeyType {
    fn from_string(string: &str) -> Self {
        match string.split(' ').collect::<Vec<&str>>()[..] {
            [val] => MonkeyType::Value(val.parse().unwrap()),
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

enum Node{
    Node(Box<Node>, String, Box<Node>),
    Leaf(i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), -1);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), -1);
    }
}
