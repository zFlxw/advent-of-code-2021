use std::fs;

fn main() {
    // File input - NOTE: If you want to use this example, you have to put an extra empty line at the end of your input. So there just be **two** empty lines
    // after the last line of the last bingo field.
    let content_string: String = fs::read_to_string("./input.txt").unwrap();
    let contents: Vec<&str> = content_string.lines().collect();

    part_01(&contents);
    part_02(&contents);
}

fn part_01(contents: &Vec<&str>) {
    let nums: Vec<usize> = contents[0].split(",").map(|n| n.to_string().parse().unwrap()).collect();
    let boards: &mut Vec<Vec<Vec<(usize, usize)>>> = &mut vec![];
    let tmp_vector: &mut Vec<Vec<(usize, usize)>> = &mut vec![];

    // Loads boards as vector
    contents.into_iter().skip(2).for_each(|line| {
        if !line.is_empty() {
            let vec: &mut Vec<(usize, usize)> = &mut vec![];
            let new_line: String = if line.chars().next().unwrap() == ' ' { line.replacen(" ", "0", 1) } else { line.to_string() };
            new_line.replace("  ", " 0").split(" ").for_each(|c| {
                vec.push((c.parse::<usize>().unwrap(), 0));
            });
            tmp_vector.push(vec.to_vec());
        } else {
            boards.push(tmp_vector.to_vec());
            tmp_vector.clear();
        }
    });

    let mut current_field: Vec<Vec<(usize, usize)>> = vec![];
    let mut winner: Vec<Vec<(usize, usize)>> = vec![];
    let mut win_num: usize = 0;
    let mut sum: usize = 0;

    // Iterate through every number.
    'outer:
    for num in &nums {
        // Update the current board and mark all fields that owns the chosen number.
        for field_index in 0..boards.len() {
            let field = &boards[field_index];
            let new_field: &mut Vec<Vec<(usize, usize)>> = &mut vec![];
            for row in field {
                let mapped: Vec<(usize, usize)> = row.into_iter().map(|col| {
                    if col.0 == *num {
                        (col.0, 1)
                    } else {
                        (col.0, col.1)
                    }
                }).collect();
                new_field.push(mapped.to_vec());
            }
            
            boards.remove(field_index);
            boards.push(new_field.to_vec());
        }

        // Evaluate, if one field has a valid row. (all numbers marked in horizontal or vertical order).
        for field_index in 0..boards.len() {
            current_field = boards[field_index].to_vec();
            for row in &current_field {
                if row.into_iter().all(|tuple| tuple.1 == 1) {
                    winner = current_field.clone();
                    win_num = *num;
                    break 'outer;
                }
            }

            for row in 0..current_field.len() {
                let mut column_count: usize = 0;
                for column in 0..current_field.len() {
                    if current_field[column][row].1 == 1 {
                        column_count += 1;
                    }
                }

                if column_count == current_field.len() {
                    winner = current_field.clone();
                    win_num = *num;
                    break 'outer;
                }
            }
        }
    }

    // Sum up all unchecked fields of winner field
    for row in &winner {
        row.into_iter().filter(|tuple| tuple.1 == 0 ).for_each(|tuple| sum += tuple.0);
    }

    println!("Nums: {:?}", &nums);
    println!("Big boi board: {:?}", boards);
    println!("Winner: {:?}", &winner);
    println!("Winner num: {}", &win_num);
    println!("Sum: {}", &sum);
    println!("Score: {}", &sum * &win_num);
}

fn part_02(contents: &Vec<&str>) {
    
}