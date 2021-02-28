#[derive(Clone, PartialEq)]
enum NodeState {
    UnReached,
    Reached,
    Finished,
}

fn dikkstra(node_count: usize, graph: &mut Vec<Vec<i32>>) -> Vec<i32> {
    let mut cost_from_start: Vec<i32> = vec![std::i32::MAX; node_count];
    let mut node_state: Vec<NodeState> = vec![NodeState::UnReached; node_count];
    // start form 0
    cost_from_start[0] = 0;

    loop {
        let mut is_unfinished_node_fount = false;
        let mut cost_of_this_node = std::i32::MAX;
        let mut cost_calc_finished_node: usize = 0;

        for node in 0..node_count {
            if node_state[node] != NodeState::Finished && cost_from_start[node] != std::i32::MAX {
                if cost_from_start[node] < cost_of_this_node{
                    cost_of_this_node = cost_from_start[node];
                    is_unfinished_node_fount = true;
                    cost_calc_finished_node = node;
                    node_state[node] = NodeState::Reached;
                }
            }
        }

        if !is_unfinished_node_fount { break; }
        node_state[cost_calc_finished_node] = NodeState::Finished;

        for node in 0..node_count {
            if node_state[node] != NodeState::Finished && graph[cost_calc_finished_node][node] != std::i32::MAX {
                if (cost_from_start[cost_calc_finished_node] + graph[cost_calc_finished_node][node]) < cost_from_start[node] {
                    cost_from_start[node] = cost_from_start[cost_calc_finished_node] + graph[cost_calc_finished_node][node];
                    node_state[node] = NodeState::Reached;
                }
            }
        }
    }
    cost_from_start
}

fn main() {
    let node_count: usize = input::read_number();
    let mut graph: Vec<Vec<i32>> = vec![vec![std::i32::MAX; node_count]; node_count];
    for _  in 0..node_count {
        let node_info: Vec<usize> = input::read_numline();
        let node = node_info[0];
        let node_branch_count = node_info[1];
        for i in 0..node_branch_count {
            let node_to = node_info[2+i*2];
            let path_cost = node_info[2+i*2+1] as i32;
            graph[node][node_to] = path_cost;
        }
    }
    let cost_from_start = dikkstra(node_count, &mut graph);
    for (node, cost) in cost_from_start.iter().enumerate() {
        println!("{} {}", node, cost);
    }
}
