use  std::collections::VecDeque;

fn breadch_search(graph_matrix: &Vec<Vec<usize>>, distance_from_node0: &mut Vec<i32>) {
    let mut breadth_queue: VecDeque<usize> = VecDeque::new();
    breadth_queue.push_back(0);
    distance_from_node0[0] = 0;

    while !breadth_queue.is_empty() {
        let node = breadth_queue.pop_front().unwrap();
        let distance = distance_from_node0[node];
        for ajucent_node in 0..distance_from_node0.len() {
            if graph_matrix[node][ajucent_node] == 1 {
                if distance_from_node0[ajucent_node] == -1 {
                    distance_from_node0[ajucent_node] = distance + 1;
                    breadth_queue.push_back(ajucent_node);
                }
            }
        }
    }

    for (node, distance) in  distance_from_node0.into_iter().enumerate() {
        println!("{} {}", node+1, distance);
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

    let mut distance_from_node0: Vec<i32> = vec![-1; nodenum];
    breadch_search(&graph_matrix, &mut distance_from_node0);
    
}
