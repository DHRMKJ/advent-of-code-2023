use std::collections::HashSet;

fn solve() {
 let input: Vec<Vec<&str>> = include_str!("test.txt")
    .lines()
    .map(|card| card.split(": ").flat_map(|val| val.split("| ")).collect())
    .collect();   

 let input_iter = input.iter();

 //skip the first value
 let mut result = 0;

 input_iter.for_each(|val|{ 
    let mut set = HashSet::new();
    let mut res = 0;
    let mut extract_val = val.iter();
    extract_val.next().unwrap();

    extract_val.next().unwrap()
        .split(" ")
        .for_each(|ch| {
            if ch.len() > 0 {
                set.insert(ch.parse::<u32>().unwrap());   
            }
        });
    
    extract_val.next().unwrap().
        split(" ")
        .for_each(|ch| {
            if ch.len() > 0 {
            let cur_num = ch.parse::<u32>().unwrap();
            let curr = set.get(&cur_num);
            match curr {
                Some(&_x) => {
                    if res == 0 {
                        res = 1;
                    }else{
                        res *= 2;
                    }
                },
                _ => {}
            }
            }
        });
        result += res;

    });

    
    

 println!("{}", result);
}


fn main() {
    solve();
}
