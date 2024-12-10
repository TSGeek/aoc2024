use std::fs;

fn part_one(content: &str) {
    let mut sum = 0;

    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for iter in re.captures_iter(content) {
        let (_, [a, b]) = iter.extract();
        sum += a.parse::<i32>().expect("Failed to parse a.")
            * b.parse::<i32>().expect("Failed to parse b.");
    }

    println!("Result of part 1 : {}", sum);
}

fn part_two(content: &str) {
    let mut sum = 0;
    let mut enabled = true;
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    for iter in re.captures_iter(content) {
        let value = iter.get(0).unwrap().as_str();
        if value == "don't()" {
            enabled = false;
        } else if value == "do()" {
            enabled = true;
        } else if value.starts_with("mul(") && enabled {
            let a = iter
                .get(1)
                .expect("Failed to get group a.")
                .as_str()
                .parse::<i32>()
                .expect("Failed to parse a.");
            let b = iter
                .get(2)
                .expect("Failed to get group b.")
                .as_str()
                .parse::<i32>()
                .expect("Failed to parse b.");
            sum += a * b;
        }
    }

    println!("Result of part 2 : {}", sum);
}

fn main() {
    let content = fs::read_to_string("inputs/day03.txt").expect("Failed to read file");
    part_one(&content);
    part_two(&content);
}
