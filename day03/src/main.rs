use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day03.txt").expect("Failed to read file");
    let mut sum = 0;

    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for iter in re.captures_iter(&content) {
        let (_, [a, b]) = iter.extract();
        sum += a.parse::<i32>().expect("Failed to parse a.") * b.parse::<i32>().expect("Failed to parse b.");
    }

    println!("Result : {}", sum);
}
