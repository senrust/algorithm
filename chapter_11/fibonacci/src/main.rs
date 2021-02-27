fn main() {
    let mut fibvec: Vec<i64> = Vec::new();
    fibvec.push(0);
    fibvec.push(1);
    for i in 2..50 {
        fibvec.push(fibvec[i - 1] + fibvec[i - 2]);
    }
    println!("40th value is {}", fibvec[40]);
}