use input::{read_number, read_numline};

fn binary_search(vec: &Vec<i32>, num: i32) -> bool {
    let mut is_found = false;
    let mut left = 0;
    let mut right = vec.len();

    while left < right {
        let mid = (left + right) / 2; 
        if vec[mid] == num {
            is_found = true;
            break;
        } else if num < vec[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    is_found
}

fn main() {
    let _: usize = read_number();
    let n_vec: Vec<i32> = read_numline();
    let _: usize = read_number();
    let s_vec: Vec<i32> = read_numline();
    let mut found_num = 0;
    for s in s_vec {
        let is_found = binary_search(&n_vec, s);
        if is_found {
            found_num += 1;
        }
    }
    println!("found count is {}", found_num);
}
