fn main() {
    let input = read_input("input.txt");

    println!("part 1: {}", part_1(&input));
    println!("part 2: \n{}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<String> {
    let text = std::fs::read_to_string(filename).expect("File not found");
    text.lines().map(|l| l.to_owned()).collect()
}

fn part_1(commands: &[String]) -> i64 {
    let mut x = 1i64;
    let mut tick = 0u32;
    let mut score = 0i64;

    for command in commands {
        match command.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => {
                score += perform_tick_1(&mut tick, &x);
            }
            ["addx", number] => {
                let number = number.parse::<i64>().expect("not a number");
                for _ in 0..2 {
                    score += perform_tick_1(&mut tick, &x);
                }
                x += number;
            }
            _ => panic!("unknown command"),
        }
    }

    score
}

fn perform_tick_1(tick: &mut u32, x: &i64) -> i64 {
    let nth_tick: i64 = (*tick + 1).into();

    *tick += 1;

    if nth_tick == 20 || (nth_tick - 20) % 40 == 0 {
        return nth_tick * *x;
    }

    0
}

fn part_2(commands: &[String]) -> String {
    let mut x = 1i64;
    let mut tick = 0u32;
    let mut result: String = "".to_owned();

    for command in commands {
        match command.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => {
                perform_tick_2(&mut tick, &x, &mut result);
            }
            ["addx", number] => {
                let number = number.parse::<i64>().expect("not a number");
                for _ in 0..2 {
                    perform_tick_2(&mut tick, &x, &mut result);
                }
                x += number;
            }
            _ => panic!("unknown command"),
        }
    }

    result
}

fn perform_tick_2(tick: &mut u32, x: &i64, result: &mut String) {
    let index: i64 = (*tick % 40).into();
    
    if index - 1 == *x || index == *x || index + 1 == *x {
        result.push('█');
    } else {
        result.push(' ');
    }
    
    *tick += 1;

    if *tick % 40u32 == 0 {
        result.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 13140);
    }

    #[test]
    fn part_2_works() {
        let input = read_input("test.txt");
        assert_eq!(
            part_2(&input),
            "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     
"
        );
    }
}
