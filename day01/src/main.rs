use std::fs;

fn part_one(content: &str) {
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();
    let mut sum = 0;
    for line in content.lines() {
        let values: Vec<&str> = line.split("   ").collect();
        left_col.push(values[0]);
        right_col.push(values[1]);
    }

    left_col.sort();
    right_col.sort();
    for i in 0..left_col.len() {
        sum += (left_col[i].parse::<i32>().unwrap() - right_col[i].parse::<i32>().unwrap()).abs();
    }
    println!("The sum is {}\n", sum);
}

fn main() {
    let content = fs::read_to_string("inputs/day01.txt").expect("Failed to read file");
    part_one(&content);
}
