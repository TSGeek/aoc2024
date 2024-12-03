use std::fs;

fn part_one(left_col: &[i32], right_col: &[i32]) {
    let mut sum = 0;
    for i in 0..left_col.len() {
        sum += (left_col[i] - right_col[i]).abs();
    }
    println!("Part 1 - The sum is {}", sum);
}

fn part_two(left_col: &[i32], right_col: &[i32]) {
    let mut sum = 0;

    for item in left_col {
        sum += item * (right_col.iter().filter(|&x| x == item).count() as i32);
    }

    println!("Part 2 - The sum is {}", sum);
}

fn main() {
    let content = fs::read_to_string("inputs/day01.txt").expect("Failed to read file");
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();
    for line in content.lines() {
        let mut values_iter = line.split("   ");
        left_col.push(values_iter.next().unwrap().parse::<i32>().unwrap());
        right_col.push(values_iter.next().unwrap().parse::<i32>().unwrap());
    }
    left_col.sort();
    right_col.sort();

    part_one(&left_col, &right_col);
    part_two(&left_col, &right_col);
}
