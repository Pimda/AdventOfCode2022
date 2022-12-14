use std::{cmp::Ordering, fmt::Display};

fn main() {
    let mut lines = read_input("input.txt");

    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&mut lines));
}

fn part_1(lines: &[String]) -> usize {
    let pairs = split_into_pairs(lines);

    let mut sum = 0;

    for (index, pair) in pairs.iter().enumerate() {
        if let [left_string, right_string] = &pair[..] {
            let left = Item::parse(&mut left_string.chars().skip(1));
            let right = Item::parse(&mut right_string.chars().skip(1));

            if left.cmp(&right) == Ordering::Less {
                sum += index + 1;
            }
        } else {
            panic!("Invalid pair")
        };
    }

    sum
}

fn part_2(lines: &mut Vec<String>) -> usize {
    let divider_1 = "[[2]]";
    let divider_2 = "[[6]]";

    lines.push(divider_1.to_owned());
    lines.push(divider_2.to_owned());

    let mut items: Vec<Item> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Item::parse(&mut l.chars()))
        .collect();

    items.sort();

    let parsed_divider_1 = Item::parse(&mut divider_1.to_string().chars());
    let parsed_divider_2 = Item::parse(&mut divider_2.to_string().chars());

    items
        .iter()
        .enumerate()
        .filter(|(_, item)| **item == parsed_divider_1 || **item == parsed_divider_2)
        .map(|(i, _)| i + 1)
        .product()
}

fn split_into_pairs(lines: &[String]) -> Vec<Vec<String>> {
    let pairs: Vec<Vec<String>> = lines
        .split(|l| l.is_empty())
        .map(|p| p.iter().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();
    pairs
}

fn parse_num(items: &mut Vec<Item>, temp: &mut String) {
    if !temp.is_empty() {
        items.push(Item::Number(temp.parse().unwrap()));
        *temp = "".to_owned()
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let string = std::fs::read_to_string(filename).expect("File not found");

    string.lines().map(|s| s.to_owned()).collect()
}

enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl Item {
    fn write(&self) -> String {
        match self {
            Item::Number(num) => num.to_string(),
            Item::List(list) => {
                "[".to_owned()
                    + &list
                        .iter()
                        .map(|i| i.write())
                        .collect::<Vec<String>>()
                        .join(",")
                    + "]"
            }
        }
    }

    fn parse(char_iter: &mut impl Iterator<Item = char>) -> Self {
        let mut items: Vec<Self> = vec![];
        let mut temp: String = "".to_owned();

        loop {
            match char_iter.next() {
                Some(',') => parse_num(&mut items, &mut temp),
                Some('[') => items.push(Self::parse(char_iter)),
                Some(']') => {
                    parse_num(&mut items, &mut temp);
                    return Self::List(items);
                }
                Some(val) => temp.push(val),
                None => return Self::List(items),
            }
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self;
        let right = other;

        match (left, right) {
            (Item::Number(l), Item::Number(r)) => l.cmp(r),
            (Item::List(left_list), Item::List(right_list)) => {
                let mut left_iter = left_list.iter();
                let mut right_iter = right_list.iter();

                loop {
                    let left = left_iter.next();
                    let right = right_iter.next();

                    if left.is_none() && right.is_none() {
                        return Ordering::Equal;
                    }

                    if left.is_none() {
                        return Ordering::Less;
                    }

                    if right.is_none() {
                        return Ordering::Greater;
                    }

                    match left.unwrap().cmp(right.unwrap()) {
                        Ordering::Equal => (),
                        ord => return ord,
                    }
                }
            }
            (Item::Number(l), Item::List(_)) => Item::List(vec![Item::Number(*l)]).cmp(right),
            (Item::List(_), Item::Number(r)) => left.cmp(&Item::List(vec![Item::Number(*r)])),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.write())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let pairs = read_input("test.txt");
        assert_eq!(part_1(&pairs), 13);
    }

    #[test]
    fn part_2_works() {
        let mut pairs = read_input("test.txt");
        assert_eq!(part_2(&mut pairs), 140);
    }

    #[test]
    fn equal_lists_are_indicisive() {
        // add a second value to force a correct ordering (indicisive and false are handled the same)
        let pairs = vec!["[[1],1]".to_owned(), "[[1],2]".to_owned()];
        assert_eq!(part_1(&pairs), 1);
    }
}
