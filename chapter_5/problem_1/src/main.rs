use input::{read_number, read_numline};

fn main() {
    let _: usize = read_number();
    let mut n_vec: Vec<i32> = read_numline();
    let _: usize = read_number();
    let s_vec: Vec<i32> = read_numline();
    let mut s_count_found_in_nvec = 0;
    for s in s_vec {
        let mut counter: usize = 0;
        let key = s;
        n_vec.push(s);
        while n_vec[counter] != key {
            counter += 1;
        }
        n_vec.pop();
        if counter != n_vec.len() {
            s_count_found_in_nvec += 1;
        }
    }
    println!("found item count is {}", s_count_found_in_nvec);
}
