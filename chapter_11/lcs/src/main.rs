use std::io;
use std::cmp::max;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let string_a = input_line.trim().parse::<String>().ok().unwrap();
    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let string_b = input_line.trim().parse::<String>().ok().unwrap();
    let mut lcs_vec: Vec<Vec<i32>> = vec![vec![0; string_b.chars().count()]; string_a.chars().count()]; 
    let mut max_lcs = 0;
    for i in 1..string_a.chars().count() {
        for j in 1..string_b.chars().count() {
            if string_a.chars().nth(i) == string_b.chars().nth(j) {
                lcs_vec[i][j] = lcs_vec[i-1][j-1] + 1;
            } else  {
                lcs_vec[i][j] = max(lcs_vec[i - 1][j], lcs_vec[i][j -1]);
            }
            max_lcs = max(max_lcs, lcs_vec[i][j]);
        }
    }
    println!("lcs is {}", max_lcs);
}
