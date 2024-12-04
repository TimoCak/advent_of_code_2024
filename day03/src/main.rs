use std::fs;

use regex::Regex;

fn main() {
    //Excercise1
    println!(
        "{:?}",
        calc_sum_of_products(save_input_as_regex_capture_list())
    );

    //Excersie2
    println!(
        "{:?}",
        calc_sum_of_products(save_enabled_instructions_as_number_list())
    );
}

fn save_input_as_regex_capture_list() -> Vec<i32> {
    let input_string = fs::read_to_string("./input.txt").expect("should read input string");
    let regex = Regex::new(r"mul\(\d{1,3},\s?\d{1,3}\)").unwrap();
    let mut results: Vec<i32> = vec![];

    let captures = regex.captures_iter(&input_string);

    let regex_number = Regex::new(r"\d{1,3}").unwrap();

    for capture in captures.map(|c| c.extract::<0>()) {
        let capture_numbers = regex_number.captures_iter(&capture.0);
        for capture_number in capture_numbers.map(|c| c.extract::<0>()) {
            let number: i32 = capture_number.0.parse().unwrap();
            results.push(number);
        }
    }

    results
}

fn calc_sum_of_products(list_of_products: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut counter1 = 0;
    let mut counter2 = 1;
    for i in 0..list_of_products.len() / 2 {
        if i == list_of_products.len() - 3 {
            break;
        }
        sum += list_of_products.get(counter1).unwrap() * list_of_products.get(counter2).unwrap();

        counter1 += 2;
        counter2 += 2;
    }
    sum
}

fn save_enabled_instructions_as_list() -> Vec<String> {
    let input_string = fs::read_to_string("./input.txt").expect("should read input string");
    let re = Regex::new(r"(don't\(\))|(mul\(\d{1,3},\s?\d{1,3}\))|(do\(\))").unwrap();
    let re_iterator = re.captures_iter(&input_string);

    let mut results: Vec<String> = vec![];
    let mut instructions_enabled = true;

    for capture in re_iterator.map(|c| c.extract::<1>()) {
        let capture_string = capture.1.get(0).unwrap().to_string();
        if capture_string.eq("don't()") {
            instructions_enabled = false;
            continue;
        } else if capture_string.eq("do()") {
            instructions_enabled = true;
            continue;
        }

        if instructions_enabled {
            results.push(capture_string);
        }
    }

    results
}

fn save_enabled_instructions_as_number_list() -> Vec<i32> {
    let regex_number = Regex::new(r"\d{1,3}").unwrap();
    let enabled_instructions = save_enabled_instructions_as_list();
    let mut results: Vec<i32> = vec![];

    for instruction in enabled_instructions {
        let capture_numbers = regex_number.captures_iter(&instruction);
        for capture_number in capture_numbers.map(|c| c.extract::<0>()) {
            let number: i32 = capture_number.0.parse().unwrap();
            results.push(number);
        }
    }
    results
}
