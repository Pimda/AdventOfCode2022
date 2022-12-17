use std::collections::{HashMap, VecDeque};

fn main() {
    let nodes = read_input("input.txt");

    println!("part 1: {}", part_1(&nodes));
}

fn part_1(nodes: &HashMap<String, Node>) -> u32 {
    let node_visit = NodeVisit {
        at_node: "AA".to_owned(),
        opened_valves: vec![],
        predicted_score: 0,
        remaining_turns: 30,
    };
    let mut node_states: HashMap<String, NodeVisit> = HashMap::new();
    node_states.insert("AA".to_owned(), node_visit.clone());

    let mut queue = VecDeque::new();
    queue.push_back(node_visit);

    loop {
        let current_node_state = queue.pop_front();

        match current_node_state {
            Some(current_node_state) => {
                if current_node_state.remaining_turns == 0 {
                    continue;
                }

                let current_node = &nodes[&current_node_state.at_node];

                // open current valve if not opened
                if !current_node_state
                    .opened_valves
                    .contains(&current_node.name)
                {
                    let node_visit = open_valve(&current_node_state, current_node);

                    if let Some(node_visit) = node_visit {
                        enqueue_if_better_score(&mut node_states, node_visit, &mut queue);
                    }
                }

                // go to all neighboring nodes
                for next_node_name in &current_node.connected_nodes {

                    let next_node = &nodes[next_node_name];
                    let new_node_visit = NodeVisit {
                        at_node: next_node.name.to_string(),
                        opened_valves: current_node_state.opened_valves.to_vec(),
                        predicted_score: current_node_state.predicted_score,
                        remaining_turns: current_node_state.remaining_turns - 1,
                    };

                    enqueue_if_better_score(&mut node_states, new_node_visit, &mut queue);
                }
            }
            None => {
                return node_states
                    .iter()
                    .map(|n| n.1.predicted_score)
                    .max()
                    .unwrap()
            }
        }
    }
}

fn open_valve(current_node_state: &NodeVisit, current_node: &Node) -> Option<NodeVisit> {
    if current_node_state.remaining_turns == 0 {
        return None;
    }

    let remaining_turns = current_node_state.remaining_turns - 1;

    let mut opened_valves = current_node_state.opened_valves.to_vec();
    opened_valves.push(current_node.name.to_string());
    let predicted_score = current_node_state.predicted_score + remaining_turns * current_node.rate;
    Some(NodeVisit {
        at_node: current_node.name.to_string(),
        opened_valves,
        predicted_score,
        remaining_turns,
    })
}

fn enqueue_if_better_score(
    node_states: &mut HashMap<String, NodeVisit>,
    node_visit: NodeVisit,
    queue: &mut VecDeque<NodeVisit>,
) {
    let state = node_states
        .entry(node_visit.at_node.to_string())
        .or_insert_with(NodeVisit::empty);

    if node_visit.predicted_score >= state.predicted_score || state.at_node.is_empty() {
        node_states
            .entry(node_visit.at_node.to_string())
            .and_modify(|e| *e = node_visit.clone());
        queue.push_back(node_visit);
    }
}

fn read_input(filename: &str) -> HashMap<String, Node> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    let nodes = string.lines().map(Node::from_string).collect::<Vec<Node>>();

    let mut hash_map = HashMap::new();

    for node in nodes {
        hash_map.insert(node.name.to_string(), node);
    }

    hash_map
}

struct Node {
    name: String,
    rate: u32,
    connected_nodes: Vec<String>,
}

impl Node {
    fn from_string(string: &str) -> Self {
        //Valve FY has flow rate=0; tunnels lead to valves TG, CD
        let [valve, connections] = string.split(';').collect::<Vec<&str>>()[..]
        else {
            panic!("error");
        };

        let valve_info = valve.split(' ').collect::<Vec<&str>>();
        let rate: u32 = valve_info
            .last()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let connections = connections.trim_start_matches(" tunnels lead to valves ");
        let connected_nodes = connections.trim_start_matches(" tunnel leads to valve ");

        Self {
            name: valve_info[1].to_owned(),
            rate,
            connected_nodes: connected_nodes.split(", ").map(|s| s.to_owned()).collect(),
        }
    }
}

#[derive(Clone)]
struct NodeVisit {
    at_node: String,
    opened_valves: Vec<String>,
    predicted_score: u32,
    remaining_turns: u32,
}

impl NodeVisit {
    fn empty() -> Self {
        Self {
            at_node: "".to_owned(),
            opened_valves: vec![],
            predicted_score: 0,
            remaining_turns: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_1(&nodes), 1651);
    }

    // #[test]
    // fn part_2_works() {
    //     let pairs = read_input("test.txt");
    //     assert_eq!(part_2(&pairs, 20), 56000011);
    // }
}
