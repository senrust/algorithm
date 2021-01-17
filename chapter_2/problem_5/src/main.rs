use std::io;
use std::str::FromStr;

fn read_number<T: FromStr>() -> T {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().parse::<T>().ok().unwrap()
}

fn read_numline<T: FromStr>() -> Vec<T>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().split_whitespace()
        .map(|num_str| num_str.parse::<T>().ok().unwrap()).collect()
}

fn read_numlines<T: FromStr>(lines: usize) -> Vec<Vec<T>> {
    (0..lines).map(|_| read_numline()).collect()
}

fn main() {
    let data_size: i32 = read_number();
    let data: Vec<i32> = (0..data_size).map(|_| read_number()).collect();
    let mut min_v = data[0];
    let mut max_v: i32 = -2_10e9 as i32;
    for i in &data[1..] {
        max_v = std::cmp::max(max_v,  *i - min_v);
        min_v = std::cmp::min(min_v, *i); 
    }
    println!("value is {}", max_v);
}
