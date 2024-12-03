use std::fs;
use crate::IsSafe::{False, True};

fn phase_one() {
    let content = fs::read_to_string("inputs/day02.txt").expect("Failed to read input file.");
    let mut safe_count = 0;
    for line in content.lines() {
        let values: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

        match check_safe(&values) {
            True => safe_count += 1,
            False(_) => (),
        }
    }
    println!("Phase 1 - Number of safe reports : {}", safe_count);
}

fn phase_two() {
    let content = fs::read_to_string("inputs/day02.txt").expect("Failed to read input file.");
    let mut safe_count = 0;
    for line in content.lines() {
        let values: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

        match check_safe(&values) {
            True => safe_count += 1,
            False(i) => {
                let mut modified_values = values.clone();
                modified_values.remove(i);
                match check_safe(&modified_values) {
                    True => safe_count += 1,
                    False(_) => {
                            let mut modified_values2 = values.clone();
                            modified_values2.remove(i+1);
                            match check_safe(&modified_values2) {
                                IsSafe::True => safe_count += 1,
                                IsSafe::False(_) => ()
                            }
                    }
                }
            }
        }
    }
    println!("Phase 2 - Number of safe reports : {}", safe_count);
}

enum IsSafe<T> {
    True,
    False(T)
}

fn check_safe(values: &[i32]) -> IsSafe<usize> {
    let is_increasing;
    if values[0] < values[1] && values[1] < values[2] {
        is_increasing = true;
    } else if  values[0] > values[1] && values[1] > values[2] {
        is_increasing = false;
    } else {
        return False(0)
    }

    // On vérifie 2 choses :
    // toujours croissant ou toujours décroissant (donc a < b ou a > b)
    // la différence toujours <= 3 et >= 0

    for i in 0..(values.len() - 1) {
        if is_increasing && values[i] < values[i + 1] {
            let diff = values[i + 1] - values[i];
            if !(1..=3).contains(&diff) {
                return False(i);
            }
        } else if !is_increasing && values[i] > values[i + 1] {
            let diff = values[i] - values[i + 1];
            if !(1..=3).contains(&diff) {
                return False(i);
            }
        } else {
            return False(i);
        }
    }

    True
}

fn main() {
    phase_one();
    phase_two();
}
