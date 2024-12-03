use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("day3\\res\\input.txt").unwrap();

    let mut sum = 0;

    //thanks to https://regex101.com/
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for line in input.lines() {
        for regex_match in regex.captures_iter(line) {
            let first_number: i32 = regex_match[1].parse().unwrap();
            let second_number: i32 = regex_match[2].parse().unwrap();
            sum += first_number * second_number;
        }
    }

    println!("sum of mul: {}", sum);

    let regex2 = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut can_mul = true;
    let mut sum2 = 0;
    for line in input.lines() {
        for regex_match in regex2.captures_iter(&line) {
            if let Some(matched) = regex_match.get(0) {
                let matched = matched.as_str();
                
                if matched.starts_with("mul") && can_mul {
                    let num1: i32 = regex_match[1].parse().unwrap();
                    let num2: i32 = regex_match[2].parse().unwrap();
                    sum2 += num1 * num2;
                } else if matched == "do()" {
                    can_mul = true;
                } else if matched == "don't()" {
                    can_mul = false;
                }
            }
        }
    }
    println!("sum of complicated mul: {}", sum2);
}
