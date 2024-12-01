use std::fs;

fn part_one(lc: &Vec<&str>, rc: &Vec<&str>) {
    let mut left_col = lc.to_vec();
    let mut right_col = rc.to_vec();
    let mut sum = 0;

    left_col.sort();
    right_col.sort();
    for i in 0..left_col.len() {
        sum += (left_col[i].parse::<i32>().unwrap() - right_col[i].parse::<i32>().unwrap()).abs();
    }
    println!("Part 1 -The sum is {}\n", sum);
}

fn part_two(left_col: &Vec<&str>, right_col: &Vec<&str>) {
    let mut sum = 0;

    for item in left_col {
        sum += item.parse::<usize>().unwrap() * right_col.iter().filter(|&x| x == item).count();
    }

    println!("Part 2 -The sum is {}\n", sum);
}

fn main() {
    let content = fs::read_to_string("inputs/day01.txt").expect("Failed to read file");
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();
    for line in content.lines() {
        let values: Vec<&str> = line.split("   ").collect();
        left_col.push(values[0]);
        right_col.push(values[1]);
    }
    part_one(&left_col, &right_col);
    part_two(&left_col, &right_col);
}
