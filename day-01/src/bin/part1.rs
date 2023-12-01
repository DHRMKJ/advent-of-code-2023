fn sum_calibration_values(words: &Vec<&str>) -> i64 {
    let mut calibration_sum: i64 = 0;
    for word in words {
        let mut first:bool = false;
        let mut last:bool = false;
        let mut calibration_val:u32 = 0;
        for (ind, ch) in word.char_indices() {
            if !first && ch.is_digit(10) {
                first = true;
                calibration_val += ch.to_digit(10).unwrap() * 10; 
            }
            let last_char = word.chars().nth(word.len()-1-ind).unwrap();
            if !last && last_char.is_digit(10) {
                last = true;
                calibration_val += last_char.to_digit(10).unwrap();
            }
        }
        calibration_sum += calibration_val as i64;
    }
    calibration_sum
}

fn main() {
    let input  = include_str!("./input.txt"); 
    let mut words: Vec<&str> = input.split('\n').collect();
    let calibration_sum = sum_calibration_values(&words);
    print!("answer :{}",calibration_sum);
}
