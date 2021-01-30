fn main() {
    let patition_num: i32 = input::read_number();
    let mut vec: Vec<i32> = input::read_numline();
    vec.push(patition_num);
    let mut border = 0;
    for j in 0..vec.len() {
        if patition_num > vec[j] {
            vec.swap(border, j);
            border += 1;
        } else{
            // nothing
        }
    }
    let last_index = vec.len() -1;
    vec.swap(border, last_index);

    println!("patitioned vec is {:?}", vec);
}
