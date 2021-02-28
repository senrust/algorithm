#[derive(Clone, PartialEq)]
enum NodeState {
    UnReached,
    Reached,
    Finished,
}

fn prim(graph: &mut Vec<Vec<i32>>, node_count: usize) -> Vec<i32> {
    let mut nodestate: Vec<NodeState> = vec![NodeState::UnReached; node_count];
    let mut next_node: Vec<i32> = vec![std::i32::MAX; node_count];
    let mut minpath_to_node: Vec<i32> = vec![std::i32::MAX; node_count];
    // start from node 0
    next_node[0] = -1;
    minpath_to_node[0] =0;

    loop {
        let mut minpath_to_unfinished_node = std::i32::MAX;
        let mut path_fnished_node: usize = 0;
        for node in 0.. node_count {
            if nodestate[node] != NodeState::Finished  && minpath_to_unfinished_node > minpath_to_node[node] {
                path_fnished_node = node;
                minpath_to_unfinished_node = minpath_to_node[node];
            }
        }

        if minpath_to_unfinished_node == std::i32::MAX {
            break;
        }

        nodestate[path_fnished_node] = NodeState::Finished;

        for node in 0..node_count {
            if nodestate[node] != NodeState::Finished && graph[path_fnished_node][node] != std::i32::MAX {
                if graph[path_fnished_node][node] < minpath_to_node[node] {
                    minpath_to_node[node] = graph[path_fnished_node][node];
                    next_node[node] = path_fnished_node as i32;
                    nodestate[node] = NodeState::Reached;
                }
            }
        }
    }
    minpath_to_node
}

fn main() {
    let node_counts: usize = input::read_number();
    let mut graph: Vec<Vec<i32>> = vec![vec![std::i32::MAX; node_counts]; node_counts]; 
    for node_index in 0..node_counts {
        let node_info: Vec<i32> = input::read_numline();
        for i in 0..node_info.len() {
            if node_info[i] == -1 {
                continue;
            }
            graph[node_index][i] = node_info[i];
        }
    }
    let minpath_to_node = prim(&mut graph, node_counts);
    let pathsum: i32 = minpath_to_node.into_iter().sum();
    println!("mininum_path sum is {}", pathsum);
}
