use std::fmt::Display;

fn main() {
    let input = read_input("input.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn read_input(filename: &str) -> Vec<Blueprint> {
    std::fs::read_to_string(filename)
        .expect("File not found")
        .lines()
        .map(Blueprint::from_string)
        .collect()
}

fn part_1(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|b| calculate_max_geodes(b, 24) * b.id)
        .sum()
}

fn part_2(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| calculate_max_geodes(b, 32))
        .product()
}

fn calculate_max_geodes(blueprint: &Blueprint, turns: u32) -> u32 {
    println!("Blueprint: {}", blueprint.id);

    let mut queue = vec![];

    let initial_ore = GameState::initial(RobotType::Ore, turns);
    let initial_clay = GameState::initial(RobotType::Clay, turns);

    queue.push(initial_ore);
    queue.push(initial_clay);

    let mut max_geodes = 0;

    loop {
        let Some(mut current_state) = queue.pop()
        else{
            break max_geodes;
        };

        match current_state.saving_for {
            RobotType::Ore => {
                if blueprint.ore_robot_cost_ore <= current_state.ore {
                    current_state.take_turn(Some(RobotType::Ore), blueprint);

                    if !could_in_theory_get_enough_geodes(&current_state, &max_geodes) {
                        continue;
                    }

                    if !finish_turn_if_last(&current_state, &mut max_geodes) {
                        add_next_steps(&mut current_state, &mut queue, blueprint, &mut max_geodes);
                    }

                    continue;
                }
            }
            RobotType::Clay => {
                if blueprint.clay_robot_cost_ore <= current_state.ore {
                    current_state.take_turn(Some(RobotType::Clay), blueprint);

                    if !could_in_theory_get_enough_geodes(&current_state, &max_geodes) {
                        continue;
                    }

                    if !finish_turn_if_last(&current_state, &mut max_geodes) {
                        add_next_steps(&mut current_state, &mut queue, blueprint, &mut max_geodes);
                    }

                    continue;
                }
            }
            RobotType::Obsidian => {
                if blueprint.obsidian_robot_cost_ore <= current_state.ore
                    && blueprint.obsidian_robot_cost_clay <= current_state.clay
                {
                    current_state.take_turn(Some(RobotType::Obsidian), blueprint);

                    if !could_in_theory_get_enough_geodes(&current_state, &max_geodes) {
                        continue;
                    }

                    if !finish_turn_if_last(&current_state, &mut max_geodes) {
                        add_next_steps(&mut current_state, &mut queue, blueprint, &mut max_geodes);
                    }

                    continue;
                }
            }
            RobotType::Geode => {
                if blueprint.geode_robot_cost_ore <= current_state.ore
                    && blueprint.geode_robot_cost_obsidian <= current_state.obsidian
                {
                    current_state.take_turn(Some(RobotType::Geode), blueprint);

                    if !could_in_theory_get_enough_geodes(&current_state, &max_geodes) {
                        continue;
                    }

                    if !finish_turn_if_last(&current_state, &mut max_geodes) {
                        add_next_steps(&mut current_state, &mut queue, blueprint, &mut max_geodes);
                    }

                    continue;
                }
            }
        }

        current_state.take_turn(None, blueprint);

        if !could_in_theory_get_enough_geodes(&current_state, &max_geodes) {
            continue;
        }

        if !finish_turn_if_last(&current_state, &mut max_geodes) {
            queue.push(current_state);
        }
    }
}

/// Very naively calculates how many geodes a state could create
/// Important to note is this method never underestimates
fn could_in_theory_get_enough_geodes(current_state: &GameState, max_geodes: &u32) -> bool {
    let mut robot_count = current_state.geode_robot_count;
    let mut geode_count = current_state.geodes;

    for _ in 0..current_state.time_remaining {
        geode_count += robot_count;
        robot_count += 1;
    }

    geode_count > *max_geodes
}

fn finish_turn_if_last(current_state: &GameState, max_geodes: &mut u32) -> bool {
    if current_state.time_remaining == 0 {
        if current_state.geodes > *max_geodes {
            *max_geodes = current_state.geodes;
            println!("Max: {}", max_geodes);
        }
        return true;
    }
    false
}

fn add_next_steps(
    current_state: &mut GameState,
    queue: &mut Vec<GameState>,
    blueprint: &Blueprint,
    max_geodes: &mut u32,
) {
    let mut pushed_value = false;

    pushed_value |= try_add_ore_robot(current_state, blueprint, queue);
    pushed_value |= try_add_clay_robot(current_state, blueprint, queue);
    pushed_value |= try_add_obsidian_robot(current_state, blueprint, queue);
    pushed_value |= try_add_geode_robot(current_state, blueprint, queue);

    if !pushed_value {
        while current_state.time_remaining > 0 {
            current_state.take_turn(None, blueprint)
        }
        finish_turn_if_last(current_state, max_geodes);
    }
}

fn try_add_ore_robot(
    current_state: &mut GameState,
    blueprint: &Blueprint,
    queue: &mut Vec<GameState>,
) -> bool {
    if current_state.ore + current_state.time_remaining * current_state.ore_robot_count
        >= blueprint.ore_robot_cost_ore
    {
        queue.push(current_state.clone_with_robot_to_build(RobotType::Ore));
        return true;
    }

    false
}

fn try_add_clay_robot(
    current_state: &mut GameState,
    blueprint: &Blueprint,
    queue: &mut Vec<GameState>,
) -> bool {
    if current_state.ore + current_state.time_remaining * current_state.ore_robot_count
        >= blueprint.clay_robot_cost_ore
    {
        queue.push(current_state.clone_with_robot_to_build(RobotType::Clay));
        return true;
    }

    false
}

fn try_add_obsidian_robot(
    current_state: &mut GameState,
    blueprint: &Blueprint,
    queue: &mut Vec<GameState>,
) -> bool {
    if current_state.ore + current_state.time_remaining * current_state.ore_robot_count
        >= blueprint.obsidian_robot_cost_ore
        && current_state.clay + current_state.time_remaining * current_state.clay_robot_count
            >= blueprint.obsidian_robot_cost_clay
        && current_state.clay_robot_count > 0
    {
        queue.push(current_state.clone_with_robot_to_build(RobotType::Obsidian));
        return true;
    }

    false
}

fn try_add_geode_robot(
    current_state: &mut GameState,
    blueprint: &Blueprint,
    queue: &mut Vec<GameState>,
) -> bool {
    if current_state.ore + current_state.time_remaining * current_state.ore_robot_count
        >= blueprint.geode_robot_cost_ore
        && current_state.obsidian
            + current_state.time_remaining * current_state.obsidian_robot_count
            >= blueprint.geode_robot_cost_obsidian
        && current_state.clay_robot_count > 0
    {
        queue.push(current_state.clone_with_robot_to_build(RobotType::Geode));
        return true;
    }

    false
}

#[derive(Clone)]
struct GameState {
    time_remaining: u32,
    saving_for: RobotType,
    ore_robot_count: u32,
    clay_robot_count: u32,
    obsidian_robot_count: u32,
    geode_robot_count: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl GameState {
    fn initial(saving_for: RobotType, turns: u32) -> Self {
        Self {
            time_remaining: turns,
            saving_for,
            ore_robot_count: 1,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
            geode_robot_count: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    fn take_turn(&mut self, robot_to_build: Option<RobotType>, blueprint: &Blueprint) {
        self.ore += self.ore_robot_count;
        self.clay += self.clay_robot_count;
        self.obsidian += self.obsidian_robot_count;
        self.geodes += self.geode_robot_count;

        if let Some(new_type) = robot_to_build {
            match new_type {
                RobotType::Ore => {
                    self.ore_robot_count += 1;
                    self.ore -= blueprint.ore_robot_cost_ore
                }
                RobotType::Clay => {
                    self.clay_robot_count += 1;
                    self.ore -= blueprint.clay_robot_cost_ore;
                }
                RobotType::Obsidian => {
                    self.obsidian_robot_count += 1;
                    self.ore -= blueprint.obsidian_robot_cost_ore;
                    self.clay -= blueprint.obsidian_robot_cost_clay;
                }
                RobotType::Geode => {
                    self.geode_robot_count += 1;
                    self.ore -= blueprint.geode_robot_cost_ore;
                    self.obsidian -= blueprint.geode_robot_cost_obsidian;
                }
            }
        }

        self.time_remaining -= 1;
    }

    fn clone_with_robot_to_build(&self, robot_to_build: RobotType) -> Self{
        let mut clone = self.clone();
        clone.saving_for = robot_to_build;

        clone
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.time_remaining == other.time_remaining
            && self.ore_robot_count == other.ore_robot_count
            && self.clay_robot_count == other.clay_robot_count
            && self.obsidian_robot_count == other.obsidian_robot_count
            && self.geode_robot_count == other.geode_robot_count
            && self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geodes == other.geodes
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geodes >= other.geodes
            && self.ore_robot_count >= other.ore_robot_count
            && self.clay_robot_count >= other.clay_robot_count
            && self.obsidian_robot_count >= other.obsidian_robot_count
            && self.geode_robot_count >= other.geode_robot_count
            && self.time_remaining >= other.time_remaining
        {
            return Some(std::cmp::Ordering::Greater);
        }

        if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geodes <= other.geodes
            && self.ore_robot_count <= other.ore_robot_count
            && self.clay_robot_count <= other.clay_robot_count
            && self.obsidian_robot_count <= other.obsidian_robot_count
            && self.geode_robot_count <= other.geode_robot_count
            && self.time_remaining <= other.time_remaining
        {
            return Some(std::cmp::Ordering::Less);
        }

        None
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ore robots:      {}, ore:      {}",
            self.ore_robot_count, self.ore
        )
        .and(writeln!(
            f,
            "clay robots:     {}, clay:     {}",
            self.clay_robot_count, self.clay
        ))
        .and(writeln!(
            f,
            "obsidian robots: {}, obsidian: {}",
            self.obsidian_robot_count, self.obsidian
        ))
        .and(writeln!(
            f,
            "geode robots:    {}, geodes:   {}",
            self.geode_robot_count, self.geodes
        ))
        .and(writeln!(f, "time remaining:  {}", self.time_remaining))
    }
}

struct Blueprint {
    //Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 9 obsidian.
    id: u32,
    ore_robot_cost_ore: u32,
    clay_robot_cost_ore: u32,
    obsidian_robot_cost_ore: u32,
    obsidian_robot_cost_clay: u32,
    geode_robot_cost_ore: u32,
    geode_robot_cost_obsidian: u32,
}

impl Blueprint {
    fn from_string(string: &str) -> Self {
        let parts = string.split(". ");

        let [part_1_and_2, part3, part4, part5] = parts.collect::<Vec<&str>>()[..]
        else{
            panic!("cannot parse parts")
        };

        let [part1, part2] = part_1_and_2.split(": ").collect::<Vec<&str>>()[..]
        else{
            panic!("cannot parse part 1 and 2")
        };

        let id = part1.split(' ').nth(1).unwrap().parse().unwrap();

        let ore_robot_cost_ore = parse_cost_one_resource(part2);
        let clay_robot_cost_ore = parse_cost_one_resource(part3);
        let (obsidian_robot_cost_ore, obsidian_robot_cost_clay) = parse_cost_two_resources(part4);
        let (geode_robot_cost_ore, geode_robot_cost_obsidian) = parse_cost_two_resources(part5);

        Self {
            id,
            ore_robot_cost_ore,
            clay_robot_cost_ore,
            obsidian_robot_cost_ore,
            obsidian_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obsidian,
        }
    }
}

#[derive(Clone, PartialEq)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse_cost_one_resource(string: &str) -> u32 {
    string.split(' ').nth(4).unwrap().parse().unwrap()
}

fn parse_cost_two_resources(string: &str) -> (u32, u32) {
    (
        string.split(' ').nth(4).unwrap().parse().unwrap(),
        string.split(' ').nth(7).unwrap().parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = read_input("test.txt");
        assert_eq!(part_1(&input), 33);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), 3472);
    }
}
