use std::collections::HashSet;

pub fn main() {
    let mut game_sum = 0;
    let mut num_scratch_cards = vec![1; DATA.lines().count()];
    for (i, line) in DATA.lines().enumerate() {
        let game = line.split_once(':').unwrap().1.trim();
        let (winning_nums, playing_nums) = game.split_once('|').unwrap();
        let mut winning_set: HashSet<i32> = HashSet::new();
        for win_num in winning_nums.split_whitespace() {
            let win_num: i32 = win_num.parse().unwrap();
            winning_set.insert(win_num);
        }

        let mut wins = 0;
        for play_num in playing_nums.split_whitespace() {
            let play_num: i32 = play_num.parse().unwrap();
            if winning_set.contains(&play_num) {
                wins += 1;
                if i + wins < num_scratch_cards.len() {
                    num_scratch_cards[i + wins] += num_scratch_cards[i];
                }
            }
        }

        game_sum += num_scratch_cards[i];
    }
    println!("{}", game_sum);
}

const DATA: &str = include_str!("../input.txt");
