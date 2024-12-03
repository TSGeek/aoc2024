use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day02.txt").expect("Failed to read input file.");
    let mut safe_count = 0;
    for line in content.lines() {
        let values: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

        let mut is_increasing = false;
        if values[0] < values[1] {
            is_increasing = true;
        }

        let mut is_safe = false;
        // On vérifie 2 choses :
        // toujours croissant ou toujours décroissant (donc a < b ou a > b)
        // la différence toujours <= 3 et >= 0

        for i in 0..(values.len()-1) {
            if is_increasing && values[i] < values[i+1] {
                let diff = values[i+1] - values[i];
                if (1..=3).contains(&diff) {
                    is_safe = true;
                } else {
                    is_safe = false;
                    break;
                }
            } else if !is_increasing && values[i] > values[i+1]  {
                let diff = values[i] - values[i+1];
                if (1..=3).contains(&diff) {
                    is_safe = true;
                } else {
                    is_safe = false;
                    break;
                }
            }
            else {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Number of safe reports : {}", safe_count);
}
