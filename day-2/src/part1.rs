use std::collections::HashMap;

pub fn main() {
    let mut max_color: HashMap<&str, i32> = HashMap::new();
    max_color.insert("red", 12);
    max_color.insert("green", 13);
    max_color.insert("blue", 14);

    let mut res = 0;
    for line in DATA.lines() {
        let (game_label, colors) = line.split_once(":").unwrap();
        let game_num: i32 = game_label
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let mut game_valid = true;
        for color_subset in colors.split(";") {
            let mut color_amount: HashMap<&str, i32> = HashMap::new();
            for color_pair in color_subset.split(",") {
                let color_pair = color_pair.trim();
                let parts: Vec<&str> = color_pair.split_whitespace().collect();
                let value: i32 = parts[0].parse().unwrap();
                let color = parts[1];

                *color_amount.entry(color).or_insert(0) += value;
            }
            for (color, max_count) in max_color.iter() {
                if *color_amount.get(color).unwrap_or(&0) > *max_count {
                    game_valid = false;
                    break;
                }
            }

            if !game_valid {
                break;
            }
        }

        if game_valid {
            res += game_num;
        }
    }
    println!("{}", res);
}

const DATA: &str = include_str!("../input.txt");
