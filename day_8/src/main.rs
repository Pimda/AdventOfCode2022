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

    check_x_direction(&forward_range, &forward_range, heights, &mut visible);
    check_x_direction(&forward_range, &reverse_range, heights, &mut visible);
    check_y_direction(&forward_range, &forward_range, heights, &mut visible);
    check_y_direction(&forward_range, &reverse_range, heights, &mut visible);

    visible
        .iter()
        .map(|line| line.iter().filter(|v| **v).count())
        .sum()
}

fn check_x_direction(
    range_y: &[usize],
    range_x: &[usize],
    heights: &[Vec<i32>],
    visible: &mut [Vec<bool>],
) {
    for y in range_y.iter() {
        let mut highest = -1;
        for x in range_x.iter() {
            check_position(heights, y, x, &mut highest, visible);
        }
    }
}

fn check_y_direction(
    range_x: &[usize],
    range_y: &[usize],
    heights: &[Vec<i32>],
    visible: &mut [Vec<bool>],
) {
    for x in range_x.iter() {
        let mut highest = -1;
        for y in range_y.iter() {
            check_position(heights, y, x, &mut highest, visible);
        }
    }
}

fn check_position(
    heights: &[Vec<i32>],
    y: &usize,
    x: &usize,
    highest: &mut i32,
    visible: &mut [Vec<bool>],
) {
    let height = heights[*y][*x];
    if height > *highest {
        visible[*y][*x] = true;
        *highest = height;
    }
}

fn part_2(heights: &[Vec<i32>]) -> u32 {
    let size = heights.len(); // input is square

    let mut scenic_score = vec![vec![0; size]; size];

    for y in 0..size {
        for x in 0..size {
            let height = heights[y][x];

            let left_range = (0..x).rev();
            let right_range = (x + 1)..size;
            let up_range = (0..y).rev();
            let down_range = (y + 1)..size;

            scenic_score[y][x] = iterate_x(left_range, heights, height, y)
                * iterate_x(right_range, heights, height, y)
                * iterate_y(up_range, heights, height, x)
                * iterate_y(down_range, heights, height, x);
        }
    }

    *scenic_score
        .iter()
        .map(|line| line.iter().max().unwrap())
        .max()
        .unwrap()
}

fn iterate_y(
    range: impl Iterator<Item = usize>,
    heights: &[Vec<i32>],
    height: i32,
    x: usize,
) -> u32 {
    let mut count = 0;
    for y in range {
        count += 1;
        if heights[y][x] >= height {
            break;
        }
    }
    count
}

fn iterate_x(
    range: impl Iterator<Item = usize>,
    heights: &[Vec<i32>],
    height: i32,
    y: usize,
) -> u32 {
    let mut count = 0;
    for x in range {
        count += 1;
        if heights[y][x] >= height {
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
