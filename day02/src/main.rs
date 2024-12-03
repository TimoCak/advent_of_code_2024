use std::fs;

fn main() {
    //Excercise01
    println!(
        "{:?}",
        evaluate_reports(&mut save_reports_as_2d_list(), false)
    );

    //Excercise02
    println!(
        "{:?}",
        evaluate_reports(&mut save_reports_as_2d_list(), true)
    );
}

fn save_reports_as_2d_list() -> Vec<Vec<i32>> {
    let input_string =
        fs::read_to_string("./input.txt").expect("should be able to read input data");
    let mut reports_2_d: Vec<Vec<i32>> = vec![];
    let mut report: Vec<i32> = vec![];
    let mut level_string: String = String::default();
    for c in input_string.chars() {
        if c != ' ' && !c.to_string().is_empty() && c != '\t' && c != '\n' {
            level_string.push(c);
            continue;
        }

        if c == ' ' {
            report.push(level_string.trim().parse::<i32>().unwrap());
            level_string = String::default();
            continue;
        }

        if c == '\n' {
            report.push(level_string.trim().parse::<i32>().unwrap());
            reports_2_d.push(report);
            level_string = String::default();
            report = vec![];
            continue;
        }
    }

    reports_2_d
}

fn is_increasing(report: &Vec<i32>) -> bool {
    for i in 0..report.len() - 1 {
        let level1 = report.get(i).unwrap().to_owned();
        let level2 = report.get(i + 1).unwrap().to_owned();
        if (level1 >= level2) || !check_increase_rule(level1, level2) {
            return false;
        }
    }
    return true;
}

fn is_decreasing(report: &Vec<i32>) -> bool {
    for i in 0..report.len() - 1 {
        let level1 = report.get(i).unwrap().to_owned();
        let level2 = report.get(i + 1).unwrap().to_owned();
        if (level1 <= level2) || !check_decrease_rule(level1, level2) {
            return false;
        }
    }
    return true;
}

fn check_decrease_rule(level1: i32, level2: i32) -> bool {
    let subtraction = level1 - level2;
    if subtraction > 0 && subtraction < 4 {
        return true;
    }
    false
}

fn check_increase_rule(level1: i32, level2: i32) -> bool {
    let subtraction = level2 - level1;
    if subtraction > 0 && subtraction < 4 {
        return true;
    }
    false
}

fn is_safe_after_remove_level(report: &Vec<i32>, index: usize) -> bool {
    let mut report_temp = report.to_owned();
    report_temp.remove(index);

    if is_increasing(&report_temp) || is_decreasing(&report_temp) {
        return true;
    }
    false
}

fn evaluate_reports(reports: &mut Vec<Vec<i32>>, dampener_enabled: bool) -> i32 {
    let mut safe_counter = 0;
    for h in 0..reports.len() {
        let report = reports.get(h).unwrap();
        if is_increasing(&report) || is_decreasing(&report) {
            safe_counter += 1;
            continue;
        }
        if dampener_enabled {
            let report_temp = report.clone();
            for i in 0..report_temp.clone().len() {
                if is_safe_after_remove_level(&report_temp, i.try_into().unwrap()) {
                    safe_counter += 1;
                    break;
                }
            }
        }
    }
    safe_counter
}
