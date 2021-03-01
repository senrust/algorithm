
use std::collections::BinaryHeap;
use std::i32;
use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
enum NodeState {
    UnReached,
    Reached,
    Finished,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    cost: i32,
    node: usize,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        // maxヒープをminヒープとして使用するためには, 
        // コストがより小さい方がGreaterとなるよう順序を反転する
        // つまり自身よりも大きいコストと比較したら, 自身がより大きい(MAXヒープで上）という判断にする
        // ノード番号は若い順でよい
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dikkstra(node_count: usize, graph: &mut Vec<Vec<(usize, i32)>>) -> Vec<i32> {
    let mut node_state: Vec<NodeState> = vec![NodeState::UnReached; node_count];
    let mut cost_calc_heap :BinaryHeap<NodeCost> = BinaryHeap::new();
    let mut cost_from_start: Vec<i32> = vec![i32::MAX; node_count];

    // start from node 0
    cost_calc_heap.push(NodeCost{cost: 0, node:0});
    cost_from_start[0] = 0;
    
    while !cost_calc_heap.is_empty() {
        let path_resolved_node = cost_calc_heap.pop().unwrap();

        // already found better path
        if path_resolved_node.cost > cost_from_start[path_resolved_node.node] { continue; }

        node_state[path_resolved_node.node] = NodeState::Finished;

        for cost_to_node in &graph[path_resolved_node.node] {
            if cost_from_start[cost_to_node.0] > cost_from_start[path_resolved_node.node] + cost_to_node.1 {
                cost_from_start[cost_to_node.0] = cost_from_start[path_resolved_node.node] + cost_to_node.1;
                node_state[cost_to_node.0] = NodeState::Reached;
                cost_calc_heap.push(NodeCost{cost: cost_from_start[cost_to_node.0] , node:cost_to_node.0})
            } 
        }
    }
    cost_from_start
}


fn main() {

    let node_count: usize = input::read_number();
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; node_count];
    for _  in 0..node_count {
        let node_info: Vec<usize> = input::read_numline();
        let node = node_info[0];
        let node_branch_count = node_info[1];
        for i in 0..node_branch_count {
            let node_to = node_info[2+i*2];
            let path_cost = node_info[2+i*2+1] as i32;
            graph[node].push((node_to, path_cost));
        }
    }
    
    let cost_from_start = dikkstra(node_count, &mut graph);
    for (node, cost) in cost_from_start.iter().enumerate() {
        println!("{} {}", node, cost);
    }
    
}
