fn main() {
    let size: i32 = input::read_number();
    let mut nums: Vec<i32> = input::read_numline();
    for i in 0..size {
        let mut min_index = i;
        let mut min_value = nums[i as usize];
        for j in i..size {
            if nums[j as usize] < min_value {
                min_index = j;
                min_value = nums[j as usize];
            }
        }
        nums.swap(i as usize, min_index as usize);
    }
    println!("order is {:?}", nums);
}
