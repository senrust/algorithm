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

fn main() {
    let heap_size: usize = input::read_number();
    let input_values: Vec<i32> = input::read_numline();
    let mut heap_vec: Vec<i32> = Vec::new();
    heap_vec.push(0);
    for i in 0..heap_size{
        heap_vec.push(input_values[i]);
    }
    for i in (1..=heap_size/2).rev() {
        maxheapfy(i, &mut heap_vec);
    }
    for i in 1..=heap_size{
        print!("{}, ", heap_vec[i])
    }
    print!("\n");
}
