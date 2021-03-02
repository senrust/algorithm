fn topological_sort(start_node: usize,  graph: &mut Vec<Vec<usize>>, 
                    order: &mut Vec<usize>, node_order_decided: &mut Vec<bool>, inflow_count: &mut Vec<usize>) {

    let mut search_node_stack: Vec<usize> = vec![];

    search_node_stack.push(start_node);

    while !search_node_stack.is_empty() {
        let decided_node = search_node_stack.pop().unwrap();
        node_order_decided[decided_node] = true;
        order.push(decided_node);

        for to_node in &graph[decided_node] {
            inflow_count[*to_node] -= 1;

            if inflow_count[*to_node] == 0 && node_order_decided[*to_node] == false {
                search_node_stack.push(*to_node);
            }
        }
    }
}

fn topological_sort_prepare(node_count: usize, graph: &mut Vec<Vec<usize>>, order: &mut Vec<usize>) {
    let mut inflow_count: Vec<usize> = vec![0; node_count];
    let mut node_order_decided: Vec<bool> = vec![false; node_count];
    for node in graph.iter() {
        for to_node in node {
            inflow_count[*to_node] += 1;
        }
    }
    for node in 0..node_count {
        if inflow_count[node] == 0 && node_order_decided[node] == false {
            topological_sort(node, graph, order, &mut node_order_decided, &mut inflow_count);
        }
    }
}

fn main() {
    let input_info: Vec<usize> = input::read_numline();
    let node_count = input_info[0];
    let input_line_count = input_info[1];
    let mut order: Vec<usize> = vec![];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; node_count];

    for _ in 0..input_line_count {
        let input_line: Vec<usize> = input::read_numline();
        let node_from = input_line[0];
        let node_to = input_line[1];
        graph[node_from].push(node_to);
    }

    topological_sort_prepare(node_count, &mut graph, &mut order);

    for node in &order {
        println!("{}", *node);
    }

}
