fn make_shell_vec(max_size: i32) -> Vec<i32> {
    let mut shell_vec: Vec<i32> =  Vec::new();
    let mut shell = 1;
    loop {
        if shell > max_size {
            break;
        }
        shell_vec.push(shell);
        shell = 3*shell + 1;
    }
    shell_vec
}

fn shell_sort(vec: &mut Vec<i32>, shell: i32, size: i32) {
    for mut index in shell..size{
        let value = vec[index as usize];
        while index >= shell &&  vec[(index - shell) as usize] >  value{
            vec.swap((index - shell) as usize, index as usize);
            index -= shell;
        }
            
    }
}

fn main() {
    let size: i32 = input::read_number();
    let mut nums: Vec<i32> = input::read_numline();
    let shell_vec = make_shell_vec(size);
    for shell in shell_vec.into_iter().rev() {
        shell_sort(&mut nums, shell, size);
    }
    println!(" order is {:?}", nums);
}
