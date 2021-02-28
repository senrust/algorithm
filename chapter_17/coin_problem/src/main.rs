fn main() {
    let input_info: Vec<usize> = input::read_numline();
    let target_value = input_info[0];
    let coins: Vec<usize> = input::read_numline();

    let mut coin_count: Vec<usize> = vec![100000000000000; target_value + 1];
    coin_count[0] = 0;
    for coin in coins {
        let mut try_value = 1;
        while try_value <= target_value {
            if 0 > try_value as isize - coin as isize {
                try_value += 1;
                continue;
            }
            if coin_count[try_value] > coin_count[try_value - coin] + 1 {
                coin_count[try_value] = coin_count[try_value - coin] + 1;
            }
            try_value += 1;
        }
    }
    println!("answer is {}", coin_count[target_value]);
}
