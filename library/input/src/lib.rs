#![allow(dead_code)]

use std::io;
use std::str::FromStr;

pub fn read_number<T: FromStr>() -> T {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().parse::<T>().ok().unwrap()
}

pub fn read_numline<T: FromStr>() -> Vec<T>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().split_whitespace()
        .map(|num_str| num_str.parse::<T>().ok().unwrap()).collect()
}

pub fn read_numlines<T: FromStr>(lines: usize) -> Vec<Vec<T>> {
    (0..lines).map(|_| read_numline()).collect()
}
