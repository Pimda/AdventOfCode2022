use std::collections::VecDeque;

type Grid<'a> = &'a[Vec<i32>];
type UPoint = (usize, usize);
type IPoint = (isize, isize);

fn main() {
    let (grid, start, end) = read_input("input.txt");

    println!("part 1: {}", part_1(&grid, start, end));
    println!("part 2: {}", part_2(&grid, start, end));
}

fn read_input(filename: &str) -> (Vec<Vec<i32>>, UPoint, UPoint) {
    let string = std::fs::read_to_string(filename).expect("Could not find file");

    let chars = &string
        .lines()
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
        })
        .collect();

    let(start, end) = find_start_and_end(chars);

    (chars.iter()
    .map(|l| l.iter()
        .map(|c| {
            match c{
                'S' => 0,
                'E' => 25,
                 _ => *c as i32 - 'a' as i32
            }
        }).collect())
        .collect(), start, end)
}

fn part_1(grid: Grid, start_index: UPoint, end_index: UPoint) -> u32 {
    let directions: Vec<IPoint> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut active_nodes = VecDeque::new();
    active_nodes.push_back(start_index);
    let mut scores: Vec<Vec<u32>> = vec![vec![u32::max_value(); grid[0].len()]; grid.len()];

    scores[start_index.1][start_index.0] = 0;

    loop {
        let active_node = active_nodes.pop_front();
        
        match active_node {
            None => break,
            Some(active_node) => {
                let current_score = scores[active_node.1][active_node.0];

                for direction in &directions {
                    let target_node = get_target_node(&active_node, direction, grid);

                    match target_node {
                        None => continue,
                        Some(target_node) => {

                            if grid[target_node.1][target_node.0] > grid[active_node.1][active_node.0] + 1{
                                continue;
                            }

                            let target_score = scores[target_node.1][target_node.0];

                            if target_score > current_score + 1 {
                                scores[target_node.1][target_node.0] = current_score + 1;
                                active_nodes.push_back(target_node);
                            }
                        }
                    }
                }
            }
        }
    }

    scores[end_index.1][end_index.0]
}

fn part_2(grid: Grid, _: UPoint, end_index: UPoint) -> u32{
    let mut min_score = u32::max_value();

    for (y, line) in grid.iter().enumerate(){
        for (x, field) in line.iter().enumerate(){
            if *field == 0{
                let score = part_1(grid, (x, y), end_index);
                if score < min_score{
                    min_score = score;
                }
            }
        }
    }

    min_score
}

fn find_start_and_end(grid: &Vec<Vec<char>>) -> (UPoint, UPoint) {
    let mut start_index: UPoint = (0, 0);
    let mut end_index: UPoint = (0, 0);

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            match char {
                'S' => start_index = (x, y),
                'E' => end_index = (x, y),
                _ => (),
            }
        }
    }

    (start_index, end_index)
}

fn get_target_node(active_node: &UPoint, direction: &IPoint, grid: Grid) -> Option<UPoint> {
    let source_x: isize = active_node.0.try_into().unwrap();
    let source_y: isize = active_node.1.try_into().unwrap();

    let target_x = source_x + direction.0;
    let target_y = source_y + direction.1;

    let target_node: IPoint = (target_x, target_y);

    if target_node.0 < 0
        || target_node.1 < 0
        || target_node.0 >= grid[0].len().try_into().unwrap()
        || target_node.1 >= grid.len().try_into().unwrap()
    {
        return None;
    }

    Some((target_x.try_into().unwrap(), target_y.try_into().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let (input, start, end) = read_input("test.txt");
        assert_eq!(part_1(&input, start, end), 31);
    }

    #[test]
    fn part_2_works() {
        let (input, start, end) = read_input("test.txt");
        assert_eq!(part_2(&input, start, end), 29);
    }
}
