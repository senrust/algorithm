// https://hnw.hatenablog.com/entry/2021/01/10/162018
// を参考に2パターンの連想配列クラスを作成する

struct Pair1 {
    key:  String,
    value :  i32,
}

struct Hash1 {
    table: [Vec<Pair1>; Hash1::TABLESIZE],
}

impl Hash1 {
    const TABLESIZE: usize = 10;

    fn new() -> Self{
        Hash1{
            // とりあえずこれで
            table: Default::default(),
        }
    }

    fn make_int_key(&self, key: &String) -> usize {
        let mut int_key: usize = 0;
        for c in key.chars() {
            int_key += c as usize;
        }
        int_key
    }

    fn hash_func (&self, int_key: usize) -> usize {
        int_key % Self::TABLESIZE
    }


    fn insert(&mut self, key: String, value: i32) {
        let int_key = self.make_int_key(&key);
        let insert_index = self.hash_func(int_key);
        for pair in self.table[insert_index].iter_mut() {
            if pair.key == key {
                pair.value = value;
            }
        }
        self.table[insert_index].push(Pair1{key: key, value: value})
    }

    fn search(&self, key: String) -> Option<i32> {
        let int_key = self.make_int_key(&key);
        let insert_index = self.hash_func(int_key);
        for pair in &self.table[insert_index] {
            if pair.key == key {
                return Some(pair.value)
            }
        }
        None
    }
}

struct Pair2 {
    key:  String,
    value :  i32,
    next: i32,
}

struct Hash2 {
    vec_index_for_table_index: usize,
    table: [i32; Hash2::TABLESIZE],
    data: Vec<Pair2>,
}

impl Hash2 {
    const TABLESIZE: usize = 10;

    fn new() -> Self{
        Hash2{
            vec_index_for_table_index: 0,
            table:[-1; Hash2::TABLESIZE],
            data: Vec::<Pair2>::new(),
        }
    }

    fn make_int_key(&self, key: &String) -> usize {
        let mut int_key: usize = 0;
        for c in key.chars() {
            int_key += c as usize;
        }
        int_key
    }

    fn hash_func (&self, int_key: usize) -> usize {
        int_key % Self::TABLESIZE

    }

    fn insert(&mut self, key: String, value: i32) {
        let int_key = self.make_int_key(&key);
        let insert_index = self.hash_func(int_key);
        if self.table[insert_index] == -1 {
            self.table[insert_index] = self.data.len() as i32;
            let insert_data = Pair2{key: key, value:value, next: -1, };
            self.data.push(insert_data);
            self.vec_index_for_table_index += 1;
        } else {
            let mut prev_index = self.table[insert_index] as usize;
            loop {
                if self.data[prev_index].key == key {
                    self.data[prev_index].value = value;
                    return;
                }
                if self.data[prev_index].next == -1 {
                    break;
                } else {
                    prev_index = self.data[prev_index].next as usize;
                }
            }
            self.data[prev_index].next = self.data.len() as i32;
            let insert_data = Pair2{key: key, value:value, next: -1, };
            self.data.push(insert_data);
        }
    }

    fn search(&self, key: String) -> Option<i32> {
        let int_key = self.make_int_key(&key);
        let insert_index = self.hash_func(int_key);
        if self.table[insert_index] == -1 {
            return None
        }
        let mut search_index = self.table[insert_index];
        while search_index != -1 {
            if self.data[search_index as usize].key == key {
                return Some(self.data[search_index as usize].value)
            }
            search_index = self.data[search_index as usize].next;
        } 
        None
    }

}

fn main() {
    let mut hash1 = Hash1::new();
    hash1.insert("Me".to_string(), 10);
    hash1.insert("Me".to_string(), 20);
    hash1.insert("You".to_string(), 15);
    println!("value is {:?} ", hash1.search("Me".to_string()));
    println!("value is {:?} ", hash1.search("You".to_string()));
    let mut hash2 = Hash2::new();
    hash2.insert("Me".to_string(), 10);
    hash2.insert("Me".to_string(), 20);
    hash2.insert("You".to_string(), 15);
    println!("value is {:?} ", hash2.search("Me".to_string()));
    println!("value is {:?} ", hash2.search("You".to_string()));
}
