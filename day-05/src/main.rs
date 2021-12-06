use std::{fs, collections::HashMap, cmp};

fn main() {
    // File input - NOTE: If you want to use this example, you have to put an extra empty line at the end of your input. So there just be **two** empty lines
    // after the last line of the last bingo field.
    let content_string: String = fs::read_to_string("./input.txt").unwrap();
    let contents: Vec<&str> = content_string.lines().collect();

    part_01(&contents);
    part_02(&contents);
}

fn part_01(contents: &Vec<&str>) {
    //                           x1     y1       x2     y2
    let sorted_input: &mut Vec<((isize, isize), (isize, isize))> = &mut vec![];
    //                                x      y       count
    let covered_points: &mut HashMap<(isize, isize), isize> = &mut HashMap::new();

    let mut highest_x: isize = 0;
    let mut highest_y: isize = 0;
    
    // Save sorted 
    for content in contents {
        let tuples: Vec<(isize, isize)> = content.split(" -> ").into_iter().map(|num| {
            let split: Vec<isize> = num.split(",").map(|n| n.parse::<isize>().unwrap()).collect();
            
            (split[0], split[1])
        }).collect();

        let tuple1: &(isize, isize) = tuples.get(0).unwrap();
        let tuple2: &(isize, isize) = tuples.get(1).unwrap();

        if tuple1.0 == tuple2.0 || tuple1.1 == tuple2.1 {
            sorted_input.push((tuple1.to_owned(), tuple2.to_owned()));
        }
        if tuple1.0 > highest_x {
            highest_x = tuple1.0.clone();
        } 
        if tuple2.0 > highest_x {
            highest_x = tuple2.0.clone();
        }
        if tuple1.1 > highest_y {
            highest_y = tuple1.1.clone();
        }
        if tuple2.1 > highest_y {
            highest_y = tuple2.1.clone();
        }
    }

    // Calculate all covered points
    for points in sorted_input.clone() {
        let x: isize = points.0.0;
        let y: isize = points.0.1;
        let dx: isize = points.1.0 - x;
        let dy: isize = points.1.1 - y;
        let n = cmp::max(dx, dy);
        let x_inc = dx / n;
        let y_inc = dy / n;

        for i in 0..n {
            let new_x = x + x_inc * i;
            let new_y = y + y_inc * i;
            let mut count = 0;
            if covered_points.contains_key(&(new_x, new_y)) {
                count = covered_points.get(&(new_x, new_y)).unwrap() + 1;
            }
            covered_points.insert((new_x, new_y), count);
        }
    }

    println!("{}", &highest_x);
    println!("{}", &highest_y);
    for x in 0..=highest_x {
        for y in 0..=highest_y {
            /* let formatted_count = if covered_points.contains_key(&(x, y)) {
                covered_points.get(&(x, y)).unwrap().to_string()
            } else { ".".into() }; */
            let count = if covered_points.contains_key(&(x, y)) {
                covered_points.get(&(x, y)).unwrap().to_string()
            } else { ".".into() };

            print!("{}", count);
        }
        println!();
    }
    
    let with_at_least_2: usize = covered_points.clone().into_iter().filter(|data| data.1 >= 2).count();

    println!("{:?}", &sorted_input);
    println!("{:?}", &covered_points);
    println!("With at least 2 {}", &with_at_least_2);
}

fn part_02(contents: &Vec<&str>) {
    
}