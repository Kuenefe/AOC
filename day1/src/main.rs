use std::{collections::HashMap, fs};

fn main() {
    //part 1
    let input: String = fs::read_to_string("day1\\res\\input.txt").unwrap();

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let mut current_number_line = line.split_whitespace();
        if let (Some(left_number), Some(right_number)) =
            (current_number_line.next(), current_number_line.next())
        {
            first_list.push(left_number.parse::<u32>().unwrap());
            second_list.push(right_number.parse::<u32>().unwrap());
        }
    }

    first_list.sort();
    second_list.sort();
    let second_list_duplicate = second_list.clone();
    
    let distance: u32 = {
        first_list
            .iter()
            .zip(second_list)
            .map(|(first_number, second_number)| first_number.abs_diff(second_number))
            .sum()
    };
    println!("this is the distance: {}", distance);
     

    //part 2

    let mut frequency_map = HashMap::new();

    for &number in &second_list_duplicate {
        if frequency_map.contains_key(&number) {
            *frequency_map.get_mut(&number).unwrap() += 1;
        } else {
            frequency_map.insert(number, 1);
        }
    }

    let mut similarity_score = 0;
    let mut similarity_tracker = Vec::new();

    for &number in &first_list {
        let number_count = *frequency_map.get(&number).unwrap_or(&0);
        similarity_score = number * number_count;
        similarity_tracker.push(similarity_score);
    }

    let result = {
        let mut tmp_res = 0;
        for score in similarity_tracker {
            tmp_res += score;
        }
        tmp_res
    };

    println!("result: {:?}", result);
}

