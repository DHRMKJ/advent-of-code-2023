use std::collections::HashMap;

fn is_special(c: char) -> bool {
    c == '*'
}

fn make_grid(i: usize, j: usize, gc: u32, grid: &mut Vec<Vec<u32>>, values: &Vec<Vec<char>>){
    const ID: [i32;3] = [-1, 0, 1];
    let mut cnt: u32 = 0;
    for x in ID {
        let ix: i32 = i as i32 + x;
        let mut curr: Vec<char> = vec![]; 
        if ix < 0 || ix >= grid.len() as i32{
            continue;
        }
        for y in ID {
            let iy: i32 = j as i32 + y;
            if iy < 0 || iy >= grid[ix as usize].len() as i32 {
                continue;
            }
            if values[ix as usize][iy as usize].is_digit(10) {
                curr.push(values[ix as usize][iy as usize]);
            }else {
                if curr.len() > 0 {
                    cnt += 1;
                    curr.clear();
                }
            }
        }
        if curr.len() > 0 {
            cnt += 1;
        }
    }
    if cnt != 2 { 
        return;
    }
    for x in ID {
        let ix: i32 = i as i32 + x;
        if ix < 0 || ix >= grid.len() as i32{
            continue;
        }
        for y in ID {
            let iy: i32 = j as i32 + y;
            if iy < 0 || iy >= grid[ix as usize].len() as i32 {
                continue;
            }
            grid[ix as usize][iy as usize] = gc;
        }
    }

}

fn valid_values(input: Vec<&str>) -> u64{
    let reinput: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();   
    let mut grid: Vec<Vec<u32>> = input.iter().map(|line| line.chars().map(|c| if is_special(c) { 1 } else { 0 } ).collect()).collect();     
    let mut gears_cnt: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_special(reinput[i][j]) {
                gears_cnt = gears_cnt + 1;
                make_grid(i, j, gears_cnt,&mut grid, &reinput);
            }
        }
    }

    let mut result: u64 = 0; 
    let mut cur_map = HashMap::new();
    for i in 0..reinput.len() {
        let mut curr: Vec<char> = vec![];
        let mut is_valid: u32 = 0;
        
        for j in 0..reinput[i].len() {
            if reinput[i][j].is_digit(10) {
                curr.push(reinput[i][j]);
                is_valid = is_valid | grid[i][j];
            }else {
                if is_valid != 0{
                    if curr.len() > 0 {
                        let num = curr.iter().collect::<String>().parse::<u64>().unwrap();
                        if cur_map.contains_key(&is_valid) {
                            let temp = cur_map.get(&is_valid).unwrap();
                            cur_map.insert(is_valid, temp * num);
                        }else {
                            cur_map.insert(is_valid, num);
                        }
                    }
                    is_valid = 0;
                }
                curr.clear();
            }
        }
        if is_valid != 0 && curr.len() > 0 {
            let num = curr.iter().collect::<String>().parse::<u64>().unwrap();
            if cur_map.contains_key(&is_valid) {
                let temp = cur_map.get(&is_valid).unwrap();
                cur_map.insert(is_valid, temp * num);
            }else {
                cur_map.insert(is_valid, num);
             }                 
        }
    }
    result += cur_map.into_values().sum::<u64>();
    result
}

fn solve() {
    let input:Vec<&str> = include_str!("input.txt").lines().collect();
    println!("{}", valid_values(input));
}



fn main() {
    solve();
}











