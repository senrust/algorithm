fn make_partition(vec: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let patition_num = vec[right -1];
    let mut border = left;
    for j in left..right {
        if patition_num > vec[j] {
            vec.swap(border, j);
            border += 1;
        } else{
            // do nothing
        }
    }
    vec.swap(border, right -1);
    border
}

fn quick_sort(vec: &mut Vec<i32>, left: usize, right: usize) {
    if left + 1 >= right {
        return;
    }
    let border = make_partition(vec, left, right);
    quick_sort(vec, left, border);
    quick_sort(vec, border + 1, right);
}

fn main() {
    let mut vec: Vec<i32> = input::read_numline();
    let vec_len = vec.len();
    quick_sort(&mut vec, 0, vec_len);
    println!("sort result is {:?}", vec);
}
