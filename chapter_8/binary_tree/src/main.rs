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

fn calc_height(nodes: &[Node], node_index: usize) -> usize {
    let mut leht_hight = 0;
    let mut right_hight = 0;
    if nodes[node_index].left_child != -1 {
        leht_hight = calc_height(nodes, nodes[node_index].left_child as usize) + 1;
    }
    if nodes[node_index].right_child != -1 {
        right_hight = calc_height(nodes, nodes[node_index].right_child as usize) + 1;
    }
    std::cmp::max(leht_hight, right_hight)
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
    let root_height = calc_height(&nodes, 0);
    println!("root height is {}", root_height);
}