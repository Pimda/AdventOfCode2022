type KeySelector = dyn Fn(usize, usize) -> (usize, usize);

fn main() {
    let input = read_input("input.txt");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string(filename).expect("File not found");
    let lines = input.lines().collect::<Vec<&str>>();

    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Invalid number") as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn part_1(heights: &[Vec<i32>]) -> usize {
    let size = heights.len(); // input is square

    let mut visible = vec![vec![false; size]; size];

    let forward_range: Vec<usize> = (0..size).collect();
    let reverse_range: Vec<usize> = (0..size).rev().collect();

    let first_y_then_x = |outer, inner| (outer, inner);
    let first_x_then_y = |outer, inner| (inner, outer);

    check_direction(
        &first_y_then_x,
        &forward_range,
        &forward_range,
        heights,
        &mut visible,
    );
    check_direction(
        &first_y_then_x,
        &forward_range,
        &reverse_range,
        heights,
        &mut visible,
    );
    check_direction(
        &first_x_then_y,
        &forward_range,
        &forward_range,
        heights,
        &mut visible,
    );
    check_direction(
        &first_x_then_y,
        &forward_range,
        &reverse_range,
        heights,
        &mut visible,
    );

    visible
        .iter()
        .map(|line| line.iter().filter(|v| **v).count())
        .sum()
}

fn check_direction(
    key_selector: &KeySelector,
    outer_range: &[usize],
    inner_range: &[usize],
    heights: &[Vec<i32>],
    visible: &mut [Vec<bool>],
) {
    for outer in outer_range.iter() {
        let mut highest = -1;
        for inner in inner_range.iter() {
            let key = key_selector(*outer, *inner);
            let height = heights[key.0][key.1];
            if height > highest {
                visible[key.0][key.1] = true;
                highest = height;
            }
        }
    }
}

fn part_2(heights: &[Vec<i32>]) -> u32 {
    let size = heights.len(); // input is square

    let mut scenic_score = vec![vec![0; size]; size];

    let iterate_x = |variable, fixed| (fixed, variable);
    let iterate_y = |variable, fixed| (variable, fixed);

    for y in 0..size {
        for x in 0..size {
            let height = heights[y][x];

            let look_left = (0..x).rev();
            let look_right = (x + 1)..size;
            let look_up = (0..y).rev();
            let look_down = (y + 1)..size;

            scenic_score[y][x] = iterate(&iterate_x, look_left, y, heights, height)
                * iterate(&iterate_x, look_right, y, heights, height)
                * iterate(&iterate_y, look_up, x, heights, height)
                * iterate(&iterate_y, look_down, x, heights, height);
        }
    }

    *scenic_score
        .iter()
        .map(|line| line.iter().max().unwrap())
        .max()
        .unwrap()
}

fn iterate(
    key_selector: &KeySelector,
    range: impl Iterator<Item = usize>,
    fixed: usize,
    heights: &[Vec<i32>],
    height: i32,
) -> u32 {
    let mut count = 0;
    for variable in range {
        count += 1;
        let key = key_selector(variable, fixed);
        if heights[key.0][key.1] >= height {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn all_visible() {
        let input = vec![vec![1, 1, 1], vec![1, 2, 1], vec![1, 1, 1]];
        assert_eq!(part_1(&input), 9);
    }

    #[test]
    fn outer_visible() {
        let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(part_1(&input), 8);
    }

    #[test]
    fn part_2_works() {
        let input = read_input("test.txt");
        assert_eq!(part_2(&input), 8);
    }
}
