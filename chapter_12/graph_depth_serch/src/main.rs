#[derive(Clone, PartialEq)]
enum ReachState {
    Unreached,
    Reached,
    AllajucentReached,
}


fn depth_search_recursive(graph_matrix: &Vec<Vec<usize>>, node_rearch_state: &mut Vec<ReachState>, node_reach_time: &mut Vec<[usize; 2]>, node: usize, time: & mut usize) {
    // check node
    *time += 1;
    node_reach_time[node][0] = *time;
    node_rearch_state[node] = ReachState::Reached;

    // search not reached ajucent node
    for ajucent_node in 0..node_rearch_state.len() {
        if graph_matrix[node][ajucent_node] == 1 && node_rearch_state[ajucent_node] == ReachState::Unreached {
            depth_search_recursive(graph_matrix, node_rearch_state, node_reach_time, ajucent_node, time);
        }
    }

    // finished reach ajucent node
    *time += 1;
    node_reach_time[node][1] = *time;
    node_rearch_state[node] = ReachState::AllajucentReached;
}


fn depth_search_stack(graph_matrix: &Vec<Vec<usize>>, node_rearch_state: &mut Vec<ReachState>, node_reach_time: &mut Vec<[usize; 2]>) {
    let mut reach_stack: Vec<usize> = vec![];
    let mut time = 1;
    reach_stack.push(0);
    node_reach_time[0][0] = time;

    while reach_stack.is_empty() == false {
        let node = *reach_stack.last().unwrap();
        let mut is_ajucent_node_found = false;
        for ajucent_node in 0..node_rearch_state.len() {
            if graph_matrix[node][ajucent_node] == 1 && node_rearch_state[ajucent_node] == ReachState::Unreached {
                time += 1;
                node_reach_time[ajucent_node][0] = time;
                node_rearch_state[ajucent_node] = ReachState::Reached;
                reach_stack.push(ajucent_node);
                is_ajucent_node_found = true;
                break;
            }
        }

        if is_ajucent_node_found == false {
            time += 1;
            let node = reach_stack.pop().unwrap();
            node_reach_time[node][1] = time;
            node_rearch_state[node] = ReachState::AllajucentReached;

        }
    }
}


fn main() {
    let nodenum: usize = input::read_number();
    let mut graph_matrix: Vec<Vec<usize>> = vec![vec![0; nodenum]; nodenum];
    for _ in 0..nodenum {
        let node_info: Vec<usize> = input::read_numline();
        let node_num = node_info[0] -1;
        let adjacent_node_count =  node_info[1];
        for i in 0..adjacent_node_count {
            let adjacent_node_num = node_info[2+i] -1;
            graph_matrix[node_num][adjacent_node_num] = 1;
        }
    }
    let mut node_reach_state: Vec<ReachState> = vec![ReachState::Unreached; nodenum];
    let mut node_reach_time: Vec<[usize; 2]> = vec![[0, 0]; nodenum];
    let mut time = 0;
    depth_search_recursive(&graph_matrix, &mut node_reach_state, &mut node_reach_time, 0, &mut time);
    
    // print all node reach time
    /* 
    for node in 0..nodenum {
        println!("{} {} {}", node, node_reach_time[node][0], node_reach_time[node][1]);
    }
    */


    let mut node_reach_state: Vec<ReachState> = vec![ReachState::Unreached; nodenum];
    let mut node_reach_time: Vec<[usize; 2]> = vec![[0, 0]; nodenum];

    depth_search_stack(&graph_matrix, &mut node_reach_state, &mut node_reach_time);
    
    // print all node reach time
    for node in 0..nodenum {
        println!("{} {} {}", node, node_reach_time[node][0], node_reach_time[node][1]);
    }
}
