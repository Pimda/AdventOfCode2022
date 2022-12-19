fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> (){
    let string =  std::fs::read_to_string(filename).expect("File not found");
}

fn part_1() -> i32{


    0
}

fn part_2() -> i32{
    

    0
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
