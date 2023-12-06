use std::collections::HashSet;

pub fn main() {
    let mut game_sum = 0;
    for line in DATA.lines() {
        let game = line.split_once(':').unwrap().1.trim();
        let (winning_nums, playing_nums) = game.split_once('|').unwrap();
        let mut winning_set: HashSet<i32> = HashSet::new();
        for win_num in winning_nums.split_whitespace() {
            let win_num: i32 = win_num.parse().unwrap();
            winning_set.insert(win_num);
        }

        let mut game_res = 0;
        for play_num in playing_nums.split_whitespace() {
            let play_num: i32 = play_num.parse().unwrap();
            if winning_set.contains(&play_num) {
                if game_res == 0 {
                    game_res = 1;
                } else {
                    game_res *= 2;
                }
            }
        }

        game_sum += game_res;
    }
    println!("{}", game_sum);
}

const DATA: &str = include_str!("../input.txt");
