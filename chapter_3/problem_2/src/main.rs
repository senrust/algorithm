fn main() {
    let size: usize = input::read_number();
    let mut nums: Vec<i32> = input::read_numline();
    let mut do_sort = true;
    let mut sord_finished_index = 1;
    while do_sort {
        do_sort = false;
        for i in (sord_finished_index..size).rev() {
            if nums[i] < nums[i-1] {
                do_sort = true;
                nums.swap(i, i-1);
            }
        } 
        sord_finished_index += 1;
    }
    println!("order is {:?}", nums);
}
