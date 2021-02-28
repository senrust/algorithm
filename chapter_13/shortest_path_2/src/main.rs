
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
struct State {
    cost: usize,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // maxヒープをminヒープとして使用するためには, 
        // コストがより小さい方がGreaterとなるよう順序を反転する
        // つまり自身よりも大きいコストと比較したら, 自身がより大きい(MAXヒープで上）という判断にする
        // ノード番号は若い順でよい
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*

fn dikkstra(node_count: usize, graph: &mut Vec<Vec<(usize, i32)>>) {
    let mut node_state: Vec<NodeState> = vec![NodeState::UnReached; node_count];
    let mut cost_from_start :BinaryHeap<(usize, i32)> = BinaryHeap::new();

    // start from node 0
    cost_from_start[0] = 0;
    
    loop {
        let mut cost_to_node = i32::MAX;
        let mut is_unfinished_node_found = false;

        for 

    }
}
*/
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
    /* 
    let cost_from_start = dikkstra(node_count, &mut graph);
    for (node, cost) in cost_from_start.iter().enumerate() {
        println!("{} {}", node, cost);
    }
    */
    
}
