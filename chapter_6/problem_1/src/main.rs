use input::{read_number, read_numline};

fn solve(vec: &Vec<i32>, index: usize, value: i32) -> bool {
    if index == vec.len() {
        return false
    }
    if vec[index] == value {
        return true
    } 
    
    let can_solve;
    can_solve = solve(vec, index + 1, value - vec[index]) | solve(vec, index + 1, value);
    can_solve
}

fn main() {
    let _: usize = read_number();
    let n_vec: Vec<i32> = read_numline();
    let _: usize = read_number();
    let s_vec: Vec<i32> = read_numline();
    let mut solved_num = 0;
    for s in s_vec{
        if solve(&n_vec, 0, s) {
            solved_num += 1;
        }
    }
    println!("solved count is {}", solved_num);

}
