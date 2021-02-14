fn maxheapfy(index: usize, heap: &mut Vec<i32>){
    let mut largest_index = index;
    if index*2 <=  heap.len() -1{
        if heap[index] < heap[index*2] {
            largest_index = index*2;
        }
    }
    if index*2+1 <=  heap.len() -1{
        if  heap[largest_index] < heap[index*2+1] {
            largest_index = index*2+1;
        }
    }
    if largest_index != index {
        heap.swap(index, largest_index);
        maxheapfy(largest_index, heap);
    }
}

fn order_insertion_key(index: usize, heap: &mut Vec<i32>) {
    if index <= 1{
        return;
    }
    while heap[index] > heap[index/2] && index > 1{
        heap.swap(index, index/2);
    }
}

fn insert(key: i32, heap: &mut Vec<i32>) {
    heap.push(key);
    order_insertion_key(heap.len() -1, heap);
}

fn extract(heap: &mut Vec<i32>) -> i32{
    let heaplen = heap.len() - 1;
    heap.swap(1, heaplen);
    let value = heap.pop().unwrap();
    maxheapfy(1, heap);
    value
}

fn main() {
    let mut heap_vec: Vec<i32> = Vec::new();
    heap_vec.push(0);
    loop {
        let input_line: Vec<String> = input::read_numline();
        if input_line[0] == "insert" {
            insert(input_line[1].parse::<i32>().ok().unwrap(), &mut heap_vec);
        } else if input_line[0] == "extract" {
            let value = extract(&mut heap_vec);
            println!("value is {}", value);
        } else {
            break;
        }
    }
}
