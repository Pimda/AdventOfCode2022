fn main() {
    let pairs = read_input("input.txt");
}

fn part_1(pairs: Vec<Vec<String>>) -> u32{

}

fn read_input(filename: &str) -> Vec<Vec<String>>{
    let string = std::fs::read_to_string(filename).expect("File not found");

    let lines: Vec<&str> = string.lines().collect();
    lines.split(|l| l.is_empty()).map(|p| p.iter().map(|s| s.to_string()).collect::<Vec<String>>()).collect()
}

enum Item{
    Number(u32),
    List(Vec<Item>)
}