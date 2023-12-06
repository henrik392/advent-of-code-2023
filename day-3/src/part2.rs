use std::collections::{HashMap, HashSet};

pub fn main() {
    let mut matrix = Vec::new();

    for line in DATA.lines() {
        let char_vector: Vec<char> = line.chars().collect();
        matrix.push(char_vector);
    }

    // For each part num:
    // Add num to vector
    // add i*line.size() + j as key to map with value of index of num in vector

    let mut part_nums: Vec<i32> = Vec::new();
    let mut part_nums_map: HashMap<usize, usize> = HashMap::new();
    let mut gears_pos: Vec<(usize, usize)> = Vec::new();

    let mut res = 0;

    for (i, line) in matrix.iter().enumerate() {
        let mut curr_num = String::new();
        let mut valid_num = false;
        for (j, &c) in line.iter().enumerate() {
            if c.is_digit(10) {
                curr_num.push(c);
                if !valid_num {
                    for &i_offset in [-1, 0, 1].iter() {
                        for &j_offset in [-1, 0, 1].iter() {
                            if let (Some(adj_i), Some(adj_j)) = (
                                (i as i32 + i_offset).try_into().ok(),
                                (j as i32 + j_offset).try_into().ok(),
                            ) {
                                let adj_i: usize = adj_i;
                                let adj_j: usize = adj_j;
                                if adj_i < matrix.len()
                                    && adj_j < matrix[adj_i].len()
                                    && !matrix[adj_i][adj_j].is_digit(10)
                                    && matrix[adj_i][adj_j] != '.'
                                {
                                    valid_num = true;
                                }
                            }
                        }
                    }
                }
            }
            if (!c.is_digit(10) || j == line.len() - 1) && !curr_num.is_empty() {
                if valid_num {
                    // Adds one so that edge case acts as default behaviour
                    let j = j + (j == line.len() - 1) as usize;

                    let part_num = curr_num.parse::<i32>().unwrap();
                    part_nums.push(part_num);

                    for num_j in [j - curr_num.len(), j] {
                        part_nums_map.insert(i * line.len() + num_j, part_nums.len() - 1);
                    }

                    valid_num = false;
                }

                curr_num.clear();
            }

            if c == '*' {
                gears_pos.push((i, j));
            }
        }
    }

    for gear_pos in gears_pos {
        let i = gear_pos.0;
        let j = gear_pos.1;
        let mut part_index_set: HashSet<usize> = HashSet::new();
        for &i_offset in [-1, 0, 1].iter() {
            for &j_offset in [-1, 0, 1].iter() {
                if let (Some(adj_i), Some(adj_j)) = (
                    (i as i32 + i_offset).try_into().ok(),
                    (j as i32 + j_offset).try_into().ok(),
                ) {
                    let adj_i: usize = adj_i;
                    let adj_j: usize = adj_j;
                    let part_key = adj_i * matrix[adj_i].len() + adj_j;
                    if adj_i < matrix.len()
                        && adj_j < matrix[adj_i].len()
                        && part_nums_map.contains_key(&(part_key))
                    {
                        part_index_set.insert(part_nums_map.get(&(part_key)).unwrap().clone());
                    }
                }
            }
        }

        if part_index_set.len() == 2 {
            let mut gear_ratio = 1;
            for part_index in part_index_set {
                let part_num = part_nums[part_index];
                gear_ratio *= part_num;
            }

            res += gear_ratio;
        }
    }

    println!("{}", res);
}

const DATA: &str = include_str!("../input.txt");
