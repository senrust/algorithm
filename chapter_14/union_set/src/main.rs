struct UnionSet {
    parent: Vec<usize>,
    depth: Vec<usize>, 
}

impl UnionSet {
    fn new(node_count: usize) -> Self {
        let mut initial_set = UnionSet {
                parent: vec![0; node_count],
                depth: vec![0; node_count],
            };
        for node in 0..node_count {
            initial_set.parent[node]= node;
        }
        initial_set
    }

    fn unite(&mut self, node1: usize, node2: usize) {
        let parent1 = self.find_most_parent(node1);
        let parent2 = self.find_most_parent(node2);

        if self.depth[parent1] == self.depth[parent2] {
            self.depth[parent2] += 1;
            self.parent[parent1] = parent2;
        } else if self.depth[parent1] > self.depth[parent2] {
            self.parent[parent2] = parent1;
        } else {
            self.parent[parent1] = parent2;
        }
    }

    fn find_most_parent(&mut self, node: usize) -> usize {
        let mut parent_node = self.parent[node];
        while self.parent[parent_node] != parent_node {
            parent_node = self.parent[parent_node];
        }
        self.parent[node] = parent_node;
        parent_node
    }

    fn comp(&mut self, node1: usize, node2: usize) -> bool {
        self.find_most_parent(node1) == self.find_most_parent(node2)
    }
}

fn main() {
    let input_info: Vec<usize> = input::read_numline();
    let node_count = input_info[0];
    let operation_count = input_info[1];
    let mut union_set = UnionSet::new(node_count);
    let mut result_vec: Vec<bool> = vec![];

    for _ in 0..operation_count {
        let operation: Vec<usize> = input::read_numline();
        if operation[0] == 0 {
            let unite_node1 = operation[1];
            let unite_node2 = operation[2];
            union_set.unite(unite_node1, unite_node2);
        } else {
            let findparent_node1 = operation[1];
            let findparent_node2 = operation[2];
            let result = union_set.comp(findparent_node1, findparent_node2);
            result_vec.push(result);
        }
    }

    for result in &result_vec {
        if *result {
            println!("1");
        } else {
            println!("0");
        }
    }
}
