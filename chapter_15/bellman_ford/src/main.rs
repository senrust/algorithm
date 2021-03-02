/// ベルマンフォードアルゴリズム理解
/// 0番目(始点)のコストは自明, 負の閉路がなければ必ず0となる
/// i番目 ノードXに到達する最短のパスは必ず始点より開始されるため,
/// 1回目のスキャンで始点の次に訪問するノードへのコストは必ず確定する
/// すなわち, どのノードへの経路でも始点からスタートする以上そのノードへの始点最小経路が求まる
/// これを繰り返すことで, 最小経路のノードが始点から近い順に求まっていく
/// V-1回目ですべてが確定する
fn bellman_ford(start_node: usize, node_count: usize, graph: &mut Vec<Vec<(usize, i32)>>, cost_from_start: &mut Vec<i32>) -> bool{
    cost_from_start[start_node] = 0;
    for i in 0..node_count {
        for (from_node, path_from_node) in graph.iter().enumerate() {
            for (to_node, cost_to) in path_from_node {
                if cost_from_start[*to_node] > cost_from_start[from_node] + cost_to {
                    // update cost
                    if i != node_count -1 {
                        cost_from_start[*to_node] = cost_from_start[from_node] + cost_to;
                    } else {
                        // this means graph has negative circuit
                        return true;
                    }
                }
            }
        }
    }
    false
}


fn main() {
    let input_info: Vec<usize> = input::read_numline();
    let node_count = input_info[0];
    let input_lines = input_info[1];
    let start_node = input_info[2];
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; node_count];
    let mut cost_from_start: Vec<i32> = vec![100000000; node_count];
    
    for _ in 0..input_lines {
        let input_line: Vec<i32> = input::read_numline();
        let from_node = input_line[0] as usize;
        let to_node = input_line[1] as usize;
        let cost = input_line[2];
        graph[from_node].push((to_node, cost));
    }
    let is_negative_cirkit = bellman_ford(start_node, node_count, &mut graph, &mut cost_from_start);
    
    if is_negative_cirkit {
        println!("NEGATIVE CIRCUIT");
        return;
    }
    
    for node in 0..node_count {
        if cost_from_start[node] == 100000000 {
            println!("INF");
        } else {
            println!("{}", cost_from_start[node]);
        }
    }
}
