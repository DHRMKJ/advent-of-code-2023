use std::collections::HashSet;

fn solve() {
 let input: Vec<Vec<&str>> = include_str!("test.txt")
    .lines()
    .map(|card| card.split(": ").flat_map(|val| val.split("| ")).collect())
    .collect();   

 let input_iter = input.iter();
 
 let mut cards: Vec<u64> = vec![1; input.len()];

 input_iter.for_each(|val|{ 
    let mut set = HashSet::new();
    let mut extract_val = val.iter();
    let mut res: usize = 0;

    let current_card_vec = extract_val.next().unwrap().split(" ").collect::<Vec<&str>>();

    let curr_card = current_card_vec[current_card_vec.len()-1].parse::<usize>().unwrap();

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
                    res += 1;
                },
                _ => {}
            }
            }
        });

    for i in 1..=res {
        cards[curr_card - 1 + i] += cards[curr_card - 1];
    }
    });
    let ans: u64 = cards.iter().sum();
   println!("{ans}"); 

}




fn main() {
    solve();
}
