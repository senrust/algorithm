use num::Bounded;

fn merge<T: Copy + Bounded + Ord>(vec: &mut Vec<T>, left: usize, right: usize, mid: usize) {
    let max_value:T  = Bounded::max_value();
    let mut vec_left = vec[left..mid].to_vec();
    let mut vec_right = vec[mid..right].to_vec();
    vec_left.push(max_value);
    vec_right.push(max_value);

    let mut left_index = 0;
    let mut right_index = 0;
    for i in left..right{
        if vec_left[left_index] > vec_right[right_index] {
            vec[i] = vec_right[right_index];
            right_index += 1;
        } else {
            vec[i] = vec_left[left_index];
            left_index += 1;
        }
    }
}

fn merge_sort<T: Copy + Bounded + Ord>(vec: &mut Vec<T>, left: usize, right: usize) {
    if left + 1 == right {
        return;
    }
    let mid = (left + right) / 2;
    merge_sort(vec, left, mid);
    merge_sort(vec, mid, right);
    merge(vec, left, right, mid);
}

fn main() {
    let mut input_vec: Vec<i32> = input::read_numline::<i32>();
    let len =  input_vec.len();
    merge_sort(& mut input_vec, 0, len);
    println!("sorted input is {:?}", input_vec);
}
