fn find_partical_graph(graph_type_num: i32, node: usize, graph_list: &mut Vec<Vec<usize>>, node_type: &mut Vec<i32>) {
    let mut search_stack: Vec<usize> = vec![];
    search_stack.push(node);
    node_type[node] = graph_type_num;

    while !search_stack.is_empty() {
        let this_node = *search_stack.last().unwrap();
        let mut is_adjust_node_found = false;
        for adjust_node in &graph_list[this_node] { 
            if node_type[*adjust_node] == -1{
                search_stack.push(*adjust_node);
                node_type[*adjust_node] = graph_type_num;
                is_adjust_node_found = true;
                break;
            }
        }
        if !is_adjust_node_found {
            search_stack.pop();
        }
    }
}

fn main() {
    let node_info: Vec<usize> = input::read_numline();
    let node_num = node_info[0];
    let input_info_lines = node_info[1];
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; node_num];
    for _ in 0..input_info_lines {
        let node_pair: Vec<usize> = input::read_numline();
        graph_list[node_pair[0]].push(node_pair[1]);
        graph_list[node_pair[1]].push(node_pair[0]);
    }
    let mut graph_type_num: i32 = 0;
    let mut node_type: Vec<i32> = vec![-1; node_num];

    for node in 0.. node_num {
        if node_type[node] == -1 {
            find_partical_graph(graph_type_num, node, &mut graph_list, &mut node_type);
            graph_type_num += 1;
        }
    }

    let assert_count: usize = input::read_number();
    for _ in 0..assert_count { 
        let relation_info: Vec<usize> = input::read_numline();
        let node0: usize  = relation_info[0];
        let node1: usize = relation_info[1];
        if node_type[node0] == node_type[node1] {
            println!("yes");
        }else {
            println!("no");
        }
    }
}
