fn solve() {
    let input = include_str!("test.txt");
    let mut ans: u64 = 0;
    let result: Vec<(u32, Vec<(u32, u32, u32)>)> = input.lines().map(|line| { 
        let temp: Vec<&str> = line.split(": ").collect();
        let game_number = temp[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let mut possible = true;
        let balls: Vec<(u32, u32, u32)> = temp[1].split("; ").collect::<Vec<&str>>().iter().map(|&round| {
            let mut red: u32 = 0;
            let mut blue: u32 = 0;
            let mut green: u32  = 0; 
            round.split(", ").collect::<Vec<&str>>().iter().for_each(|colour| {
                    let ball_instance = colour.split(" ").collect::<Vec<&str>>();
                    let cur_color: &str = ball_instance[1];
                    let cur_cnt : u32 = ball_instance[0].parse::<u32>().unwrap();
                    match cur_color {
                        "red" => { red += cur_cnt},
                        "green" => { green += cur_cnt},
                        "blue" => { blue += cur_cnt},
                        _ => panic!("something is wrong with the input")
                    }
                });
            if red > 12 || green > 13 || blue > 14 {
                possible = false;
            }
            (red, blue, green)
        }).collect();
        println!("balls: {:?}", balls);

        if possible {
            ans += game_number as u64;
        }
        (game_number, balls)
    }).collect();
    println!("{ans}");
}

fn main() {
    solve();
}
