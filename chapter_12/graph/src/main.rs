use std::usize;

fn main() {
    let nodesize: usize = input::read_number();
    let mut graph_matrix: Vec<Vec<usize>> = vec![vec![0; nodesize]; nodesize];
    for _ in 0..nodesize {
        let node_info: Vec<usize> = input::read_numline();
        let node_num = node_info[0] -1;
        let adjacent_node_count =  node_info[1];
        for i in 0..adjacent_node_count {
            let adjacent_node_num = node_info[2+i] -1;
            graph_matrix[node_num][adjacent_node_num] = 1;
        }
    }
    for node in graph_matrix {
        for isadjacent in node {
            print!("{} ", isadjacent);
        }
        print!("\n")
    }
}
