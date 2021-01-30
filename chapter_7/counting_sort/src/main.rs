fn main() {
    let vec: Vec<i32> = input::read_numline();
    let max_value = vec.iter().max().unwrap();
    let mut count_vec = vec![0; *max_value as usize + 1];
    for i in vec {
        count_vec[i as usize] += 1;
    }
    let mut sorted_vec = Vec::<i32>::new();
    for i in 0..count_vec.len() {
        if count_vec[i] != 0 {
            for _ in 0..count_vec[i]{
                sorted_vec.push(i as i32);
            }
        }
    }
    println!("sorted vec is {:?}", sorted_vec);
}
