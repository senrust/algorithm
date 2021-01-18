use input::read_number;

fn main() {
    let size: i32 = read_number();
    let mut nums: Vec<i32> = input::read_numline();
    for i in 1..size {
        let value = nums[i as usize];
        let mut j = i - 1;
        while j >= 0 && nums[j as usize] > value  {
            nums.swap(j as usize, (j + 1) as usize);
            j -= 1;
        }
    }
    println!("order is {:?}", nums);
}
