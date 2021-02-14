
fn main() {
    let heap_size: usize = input::read_number();
    let input_values: Vec<i32> = input::read_numline();
    let mut heap_vec: Vec<Option<i32>> = Vec::new();
    heap_vec.push(None);
    for i in 0..heap_size {
        let value: i32 = input_values[i];
        heap_vec.push(Some(value));
    }

    for i in 1..heap_size+1{
        print!("node {}: ", i);
        print!("key = {}, ", heap_vec[i].unwrap());
        if i != 1 {
            print!("parent key = {}, ", heap_vec[i/2].unwrap());
        }
        if 2*i <= heap_size {
            print!("left key = {}, ", heap_vec[2*i].unwrap());
        }
        if 2*i+1 <= heap_size {
            print!("right key = {}, ", heap_vec[2*i+1].unwrap());
        }
        print!("\n");
    }
}
