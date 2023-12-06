pub fn main() {
    let mut matrix = Vec::new();

    for line in DATA.lines() {
        let char_vector: Vec<char> = line.chars().collect();
        matrix.push(char_vector);
    }

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
            } else if !curr_num.is_empty() {
                if valid_num {
                    res += curr_num.parse::<i32>().unwrap();
                }
                curr_num.clear();
                valid_num = false;
            }
            if j == line.len() - 1 && valid_num {
                res += curr_num.parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", res);
}

const DATA: &str = include_str!("../input.txt");
