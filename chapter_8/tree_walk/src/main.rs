#[derive(Copy, Clone)]
struct Node {
    parent_node: i32,
    left_child: i32,
    right_child: i32,
}

impl Default for Node {
    fn default() -> Self { 
        Node{
            parent_node: -1,
            left_child: -1,
            right_child: -1,
        }
    }
}

fn preorder_walk(nodes: &[Node], node_index: i32) {
    if node_index == -1 {
        return;
    }
    print!("{} ", node_index);
    preorder_walk(nodes, nodes[node_index as usize].left_child);
    preorder_walk(nodes, nodes[node_index as usize].right_child);
}

fn inorder_walk(nodes: &[Node], node_index: i32) {
    if node_index == -1 {
        return;
    }
    preorder_walk(nodes, nodes[node_index as usize].left_child);
    print!("{} ", node_index);
    preorder_walk(nodes, nodes[node_index as usize].right_child);
}

fn postorder_walk(nodes: &[Node], node_index: i32) {
    if node_index == -1 {
        return;
    }
    preorder_walk(nodes, nodes[node_index as usize].left_child);
    preorder_walk(nodes, nodes[node_index as usize].right_child);
    print!("{} ", node_index);
}

fn main() {
    let mut nodes: [Node; 20]  = [Default::default(); 20];
    let node_num: usize = input::read_number();
    for _ in 0..node_num{
        let node_info: Vec<i32> = input::read_numline();
        let node_id = node_info[0];
        let left_child_id = node_info[1];
        let right_child_id = node_info[2];
        if left_child_id != -1 {
            nodes[node_id as usize].left_child = left_child_id;
            nodes[left_child_id as usize].parent_node = node_id;
        }
        if right_child_id != -1 {
            nodes[node_id as usize].right_child = right_child_id;
            nodes[right_child_id as usize].parent_node = node_id;
        }
    }
    preorder_walk(&nodes, 0);
    print!("\n");
    inorder_walk(&nodes, 0);
    print!("\n");
    postorder_walk(&nodes, 0);
    print!("\n");
}