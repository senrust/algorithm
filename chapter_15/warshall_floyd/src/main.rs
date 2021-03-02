fn warshall_ford(node_count: usize, cost_to_node: &mut Vec<Vec<i64>>) {
    for interchange_node in 0..node_count {
        for node_from in 0..node_count {
            for node_to in 0..node_count {
                let cost_by_interchange: i64 = cost_to_node[node_from][interchange_node] as i64 + cost_to_node[interchange_node][node_to] as i64;
                if cost_to_node[node_from][node_to] > cost_by_interchange {
                    cost_to_node[node_from][node_to] = cost_by_interchange;
                }
            }
        }
    }
}

fn main() {
    let input_info: Vec<usize> = input::read_numline();
    let node_count = input_info[0];
    let input_lines = input_info[1];
    let mut cost_to_node: Vec<Vec<i64>> = vec![vec![i32::MAX as i64; node_count]; node_count];

    for _ in 0..input_lines {
        let node_relation: Vec<i64> = input::read_numline();
        let node1 = node_relation[0] as usize;
        let node2 = node_relation[1]  as usize;
        let cost = node_relation[2] as i64;
        cost_to_node[node1][node2] = cost;
    }
    for node in 0..node_count {
        cost_to_node[node][node] = 0;
    }

    warshall_ford(node_count, &mut cost_to_node);
    for node in 0..node_count {
        if cost_to_node[node][node] < 0 {
            println!("NEGATIVE CYCLE");
            return;
        }
    }

    for cost_vec in &cost_to_node {
        for cost in cost_vec {
            if *cost == i32::MAX as i64{
                print!("INF ");
            } else {
                print!("{} ", *cost);
            }
        }
        print!("\n");
    }
}
