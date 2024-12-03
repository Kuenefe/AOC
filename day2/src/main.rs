use std::fs;

fn main() {
    let input: String = fs::read_to_string("day2\\res\\input.txt").unwrap();

    let mut all_reports: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let single_report = line
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        all_reports.push(single_report);
    }

    let mut safe_reports: Vec<Vec<i32>> = Vec::new();
    let mut unsafe_reports: Vec<Vec<i32>> = Vec::new();

    for report in all_reports {
        let mut saved_direction: Option<i32> = None;
        let mut is_safe = true;

        for i in 0..report.len() - 1 {
            let current_number = report[i];
            let next_number = report[i + 1];
            let diff = next_number - current_number;

            if diff.abs() < 1 || diff.abs() > 3 {
                is_safe = false;
                break;
            }

            let current_direction = diff.signum();

            if saved_direction.is_none() {
                saved_direction = Some(current_direction);
            } else if saved_direction.unwrap() != current_direction {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe_reports.push(report);
        } else {
            let mut make_report_safe_again = false;
            for i in 0..report.len() {
                let mut tmp_report = report.clone();
                tmp_report.remove(i);

                if is_report_safe(&tmp_report) {
                    make_report_safe_again = true;
                    break;
                }
            }

            if make_report_safe_again {
                safe_reports.push(report);
            } else {
                unsafe_reports.push(report);
            }
        }
    }

    println!("safe report count: {}", safe_reports.len());
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut saved_direction: Option<i32> = None;

    for i in 0..report.len() - 1 {
        let current_number = report[i];
        let next_number = report[i + 1];
        let diff = next_number - current_number;

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        let current_direction = diff.signum();

        if saved_direction.is_none() {
            saved_direction = Some(current_direction);
        } else if saved_direction.unwrap() != current_direction {
            return false;
        }
    }

    true
}
