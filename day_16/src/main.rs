use std::collections::{HashMap, VecDeque};

fn main() {
    let nodes = read_input("input.txt");

    println!("part 1: {}", part_1(&nodes));
    println!("part 2: {}", part_2(&nodes));
}

/// Now that the graph is simplified, this part can be handled mostly brute force
fn part_1(nodes: &HashMap<String, Node>) -> i32 {
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

        if let Some(current_node_state) = current_node_state {
            handle_step(nodes, current_node_state, &mut node_states, &mut queue);
        } else {
            return node_states
                .values()
                .map(|node| node.predicted_score)
                .max()
                .unwrap();
        }
    }
}

/// First determine all possible outcomes for 'me', and then let the elephant continue with those states
/// right now there are a bit more than 40.000 states, these must be able to be filtered,
/// but it turns out the right answer is found within the first 10% of those states.
fn part_2(nodes: &HashMap<String, Node>) -> i32 {
    let node_visit = NodeVisit {
        at_node: "AA".to_owned(),
        opened_valves: vec![],
        predicted_score: 0,
        remaining_turns: 26,
    };
    let mut node_states: HashMap<String, NodeVisit> = HashMap::new();
    node_states.insert("AA".to_owned(), node_visit.clone());

    let mut queue = VecDeque::new();
    queue.push_back(node_visit);

    loop {
        let current_node_state = queue.pop_front();

        if let Some(current_node_state) = current_node_state {
            handle_step(nodes, current_node_state, &mut node_states, &mut queue);
        } else {
            break;
        }
    }

    let mut max_score = 0;

    println!(
        "Done with me, now continuing with elephant for {} states",
        node_states.len()
    );

    for (i, state) in node_states.iter().enumerate() {
        let node_visit = NodeVisit {
            at_node: "AA".to_owned(),
            opened_valves: state.1.opened_valves.to_vec(),
            predicted_score: state.1.predicted_score,
            remaining_turns: 26,
        };
        let mut node_states: HashMap<String, NodeVisit> = HashMap::new();
        node_states.insert("AA".to_owned(), node_visit.clone());

        let mut queue = VecDeque::new();
        queue.push_back(node_visit);

        loop {
            let current_node_state = queue.pop_front();

            if let Some(current_node_state) = current_node_state {
                handle_step(nodes, current_node_state, &mut node_states, &mut queue);
            } else {
                let score = node_states
                    .values()
                    .map(|node| node.predicted_score)
                    .max()
                    .unwrap();
                if score > max_score {
                    max_score = score;
                }
                break;
            }
        }

        if i % 1000 == 0 {
            println!(
                "Finished state {} for elephant, current max score {}",
                i + 1,
                max_score
            );
        }
    }

    max_score
}

fn handle_step(
    nodes: &HashMap<String, Node>,
    current_node_state: NodeVisit,
    node_states: &mut HashMap<String, NodeVisit>,
    my_queue: &mut VecDeque<NodeVisit>,
) {
    let current_node = &nodes[&current_node_state.at_node];
    // go to all neighboring nodes
    for edge in &current_node.connected_nodes {
        let next_node = &nodes[&edge.connected_to];

        if current_node_state
            .opened_valves
            .contains(&edge.connected_to)
        {
            continue;
        }

        let remaining_turns = current_node_state.remaining_turns - edge.cost - 1;

        if remaining_turns <= 0 {
            continue;
        }

        // Note: always open the valve we're visiting, it makes no senso to go to a valve and leave it closed
        let next_flow = nodes[&edge.connected_to].rate;
        let predicted_score = current_node_state.predicted_score + next_flow * remaining_turns;

        let mut opened_valves = current_node_state.opened_valves.to_vec();
        opened_valves.push(edge.connected_to.clone());

        let new_node_visit = NodeVisit {
            at_node: next_node.name.clone(),
            opened_valves,
            predicted_score,
            remaining_turns,
        };
        enqueue_if_better_score(node_states, new_node_visit, my_queue);
    }
}

fn enqueue_if_better_score(
    node_states: &mut HashMap<String, NodeVisit>,
    node_visit: NodeVisit,
    queue: &mut VecDeque<NodeVisit>,
) {
    let key = node_visit.at_node.clone() + "-" + &node_visit.opened_valves.join(",");

    node_states.entry(key).or_insert_with(|| node_visit.clone());
    queue.push_back(node_visit);
}

fn read_input(filename: &str) -> HashMap<String, Node> {
    let string = std::fs::read_to_string(filename).expect("File not found");
    let nodes = string.lines().map(Node::from_string).collect::<Vec<Node>>();

    let mut hash_map = HashMap::new();

    for node in nodes {
        hash_map.insert(node.name.to_string(), node);
    }

    simplify_graph(hash_map)
}

/// Returns only the start node and nodes with flow
fn simplify_graph(hash_map: HashMap<String, Node>) -> HashMap<String, Node> {
    let mut simple_graph = HashMap::new();

    let nodes_with_flow: Vec<String> = hash_map
        .values()
        .filter(|n| n.rate > 0)
        .map(|n| n.name.clone())
        .collect();

    simple_graph.insert(
        "AA".to_owned(),
        simplify_node(&hash_map, &nodes_with_flow, "AA".to_owned()),
    );

    for name in &nodes_with_flow {
        simple_graph.insert(
            name.clone(),
            simplify_node(&hash_map, &nodes_with_flow, name.clone()),
        );
    }

    simple_graph
}

/// For the given node find all travel lengths to nodes with flow
fn simplify_node(
    hash_map: &HashMap<String, Node>,
    nodes_with_flow: &Vec<String>,
    start_name: String,
) -> Node {
    let start_node = &hash_map[&start_name];
    let mut new_node = Node {
        name: start_node.name.clone(),
        connected_nodes: vec![],
        rate: start_node.rate,
    };
    let distances = dijkstra(hash_map, start_node.name.clone());

    for name in nodes_with_flow {
        if &start_name == name {
            continue;
        }

        let distance = &distances[name];
        let edge = Edge {
            connected_to: name.clone(),
            cost: *distance,
        };

        new_node.connected_nodes.push(edge);
    }

    new_node
}

/// For the whole graph find the shortest distances to the start node
fn dijkstra(nodes: &HashMap<String, Node>, start_node: String) -> HashMap<String, i32> {
    let mut distances = HashMap::new();
    distances.insert(start_node.clone(), i32::max_value());

    let mut queue = VecDeque::new();
    queue.push_back(DijkstraVisit {
        at_node: start_node,
        cost: 0,
    });

    loop {
        let current_visit = queue.pop_front();

        if let Some(current_visit) = current_visit {
            let distance = &distances
                .entry(current_visit.at_node.clone())
                .or_insert_with(i32::max_value);

            if current_visit.cost < **distance {
                distances
                    .entry(current_visit.at_node.clone())
                    .and_modify(|e| *e = current_visit.cost);

                let node = &nodes[&current_visit.at_node];

                for connected_node in &node.connected_nodes {
                    queue.push_back(DijkstraVisit {
                        at_node: connected_node.connected_to.clone(),
                        cost: current_visit.cost + 1,
                    })
                }
            }
        } else {
            break;
        }
    }

    distances
}

struct Node {
    name: String,
    rate: i32,
    connected_nodes: Vec<Edge>,
}

impl Node {
    fn from_string(string: &str) -> Self {
        //Valve FY has flow rate=0; tunnels lead to valves TG, CD
        let [valve, connections] = string.split(';').collect::<Vec<&str>>()[..]
        else {
            panic!("error");
        };

        let valve_info = valve.split(' ').collect::<Vec<&str>>();
        let rate: i32 = valve_info
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
            connected_nodes: connected_nodes
                .split(", ")
                .map(|s| Edge {
                    connected_to: s.to_owned(),
                    cost: 1,
                })
                .collect(),
        }
    }
}

struct Edge {
    connected_to: String,
    cost: i32,
}

struct DijkstraVisit {
    at_node: String,
    cost: i32,
}

#[derive(Clone)]
struct NodeVisit {
    at_node: String,
    opened_valves: Vec<String>,
    predicted_score: i32,
    remaining_turns: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_1(&nodes), 1651);
    }

    #[test]
    fn part_2_works() {
        let nodes = read_input("test.txt");
        assert_eq!(part_2(&nodes), 1707);
    }
}
