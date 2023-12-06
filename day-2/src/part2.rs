use std::{cmp, collections::HashMap};

pub fn main() {
    let colors = vec!["red", "green", "blue"];

    let mut res = 0;
    for line in DATA.lines() {
        let mut min_cubes: HashMap<&str, i32> = HashMap::new();

        let (_, game) = line.split_once(":").unwrap();

        for color_subset in game.split(";") {
            let mut color_amount: HashMap<&str, i32> = HashMap::new();
            for color_pair in color_subset.split(",") {
                let color_pair = color_pair.trim();
                let parts: Vec<&str> = color_pair.split_whitespace().collect();
                let value: i32 = parts[0].parse().unwrap();
                let color = parts[1];

                *color_amount.entry(color).or_insert(0) += value;
            }
            for &color in &colors {
                *min_cubes.entry(color).or_insert(0) = cmp::max(
                    *min_cubes.get(color).unwrap_or(&0),
                    *color_amount.get(color).unwrap_or(&0),
                );
            }
        }

        let mut game_res = 1;
        for &color in &colors {
            game_res *= *min_cubes.get(color).unwrap_or(&0);
        }

        res += game_res;
    }
    println!("{}", res);
}

const DATA: &str = include_str!("../input.txt");
