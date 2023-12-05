fn is_special(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

fn make_grid(i: usize, j: usize, grid: &mut Vec<Vec<bool>>){
    const ID: [i32;3] = [-1, 0, 1];
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
            grid[ix as usize][iy as usize] = true;
        }
    }
}

fn valid_values(input: Vec<&str>) -> u64{
    let reinput: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();   
    let mut grid: Vec<Vec<bool>> = input.iter().map(|line| line.chars().map(|c| is_special(c)).collect()).collect();     
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_special(reinput[i][j]) {
                make_grid(i, j,&mut grid);
            }
        }
    }
    let mut result: u64 = 0; 
    
    for i in 0..reinput.len() {
        let mut curr: Vec<char> = vec![];
        let mut cur_list: Vec<Vec<char>> = vec![vec![]];
        let mut is_valid = false;

        for j in 0..reinput[i].len() {
            if reinput[i][j].is_digit(10) {
                curr.push(reinput[i][j]);
                is_valid = is_valid | grid[i][j];
            }else {
                if is_valid {
                    is_valid = false;
                    cur_list.push(curr.clone());
                }
                curr.clear();
            }
        }
        if is_valid && curr.len() > 0 {
            cur_list.push(curr.clone());
        }

        let num: u64 = cur_list.iter().map(|cur| cur.iter().collect::<String>().parse::<u64>().unwrap_or_else(|_| 0)).sum();
        result += num;
    }

    result
}

fn solve() {
    let input:Vec<&str> = include_str!("input.txt").lines().collect();
    println!("{}", valid_values(input));
}



fn main() {
    solve();
}
