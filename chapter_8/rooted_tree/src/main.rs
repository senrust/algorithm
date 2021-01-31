#[derive(Copy, Clone)]
struct Node {
    parent_node: i32,
    brother_right: i32,
    child_left: i32,
}

impl Default for Node {
    fn default() -> Self { 
        Node{
            parent_node: -1,
            brother_right: -1,
            child_left: -1,
        }
    }
}

fn calc_depth(nodes: &[Node], depth: &mut [usize], node_num: i32, node_depth: usize) { 
    depth[node_num as usize] = node_depth;
    // 子ノードが存在すれば
    if nodes[node_num as usize].child_left != -1 {
        let child_node = nodes[node_num as usize].child_left;
        calc_depth(nodes, depth, child_node, node_depth + 1);
    }
    // 弟ノードが存在すれば
    if nodes[node_num as usize].brother_right != -1 {
        let brother_node = nodes[node_num as usize].brother_right;
        calc_depth(nodes, depth, brother_node, node_depth);
    }

}

fn print_nodes(nodes: &[Node], depth: &[usize], node_num: usize) {
    for i in 0..node_num {
        let node = nodes[i];
        print!("node {}: parent = {}, depth: {}, ", i, node.parent_node, depth[i]);
        if node.parent_node == -1 {
            print!("root node, ");
        } else if node.child_left == -1 {
            print!("leef node, ");
        } else {
            print!("internal node, ");
        }
        print!("\n");
    }
}

fn main() {
    let mut nodes: [Node; 20]  = [Default::default(); 20];
    let mut depth: [usize; 20]  = [0; 20];
    let node_num: usize = input::read_number();
    for _ in 0..node_num{
        let node_info: Vec<usize> = input::read_numline();
        let node_id = node_info[0];
        let child_num = node_info[1];
        let mut child_left = 0;
        for child_index in 0..child_num {
            let child_id = node_info[child_index + 2];
            if child_index == 0 {
                nodes[node_id].child_left = child_id as i32;
            } else {
                nodes[child_left].brother_right = child_id as i32;
            }
            child_left = child_id;
            nodes[child_id].parent_node = node_id as i32;
        }
    }
    
    let mut root_node: i32 = -1;
    for i in 0..node_num {
        if nodes[i].parent_node == -1 {
            root_node = i as i32;
            break;
        }
    }

    calc_depth(&nodes, &mut depth, root_node, 0);
    print_nodes(&nodes, &depth, node_num);
}
