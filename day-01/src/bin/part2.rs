fn sum_calibration_values(words: &Vec<String>) -> i64 {
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
fn make_it_simple(mut word: &str) -> String {
    let mut word = word.to_owned();
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i, number) in numbers.iter().enumerate() {
        let ind: usize = i + 1;
        word = word.replace(number, format!("{}{}{}", number, ind, number).as_str());
    }
    word
}


fn calibration_again(words: Vec<&str>) -> i64 {
    let result: Vec<String> = words.iter().map(|word| {
        println!("{word}");
        let word:String = make_it_simple(word);
        word
    }).collect();
    sum_calibration_values(&result)
}

fn main() {
    let input  = include_str!("./input.txt"); 
    let words: Vec<&str> = input.split('\n').collect();
    let calibration_sum = calibration_again(words);
    print!("answer :{}",calibration_sum);
}
