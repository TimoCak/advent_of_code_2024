use std::fs;

const XMAS: &str = "XMAS";

pub fn save_input_into_2_d_list() -> Vec<Vec<char>> {
    let mut input_string = fs::read_to_string("./input.txt").expect("should read input file");
    input_string = input_string.replace("\r", "");

    let mut list_2_d: Vec<Vec<char>> = vec![];
    let mut char_list: Vec<char> = vec![];

    for c in input_string.chars() {
        if c == '\n' {
            list_2_d.push(char_list);
            char_list = vec![];
            continue;
        }
        char_list.push(c);
    }
    list_2_d
}

fn horizontal_count(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> i32 {
    let mut amount = 0;
    let mut current_string = String::default();
    for i in 0..XMAS.len() {
        let coord = list_2_d.get(x).unwrap().get(y + i);
        if coord.is_some() {
            current_string.push(coord.unwrap().to_owned());
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    current_string = String::default();
    for i in 0..XMAS.len() {
        if y >= i {
            let coord = list_2_d.get(x).unwrap().get(y - i);
            if coord.is_some() {
                current_string.push(coord.unwrap().to_owned());
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    amount
}

fn vertical_count(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> i32 {
    let mut amount = 0;
    let mut current_string = String::default();
    for i in 0..XMAS.len() {
        let check_x = list_2_d.get(x + i);
        if check_x.is_some() {
            let coord = check_x.unwrap().get(y);
            current_string.push(coord.unwrap().to_owned());
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    current_string = String::default();
    for i in 0..XMAS.len() {
        if x >= i {
            let check_x = list_2_d.get(x - i);
            if check_x.is_some() {
                let coord = check_x.unwrap().get(y);
                current_string.push(coord.unwrap().to_owned());
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    amount
}

fn diagonal_count(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> i32 {
    let mut amount = 0;
    let mut current_string = String::default();
    for i in 0..XMAS.len() {
        let check_x = list_2_d.get(x + i);
        if check_x.is_some() {
            let check_y = check_x.unwrap().get(y + i);
            if check_y.is_some() {
                let coord = check_y.unwrap();
                current_string.push(coord.to_owned());
            }
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    current_string = String::default();

    for i in 0..XMAS.len() {
        let check_x = list_2_d.get(x + i);
        if y >= i {
            if check_x.is_some() {
                let check_y = check_x.unwrap().get(y - i);
                if check_y.is_some() {
                    let coord = check_y.unwrap();
                    current_string.push(coord.to_owned());
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    current_string = String::default();

    for i in 0..XMAS.len() {
        if x >= i {
            let check_x = list_2_d.get(x - i);
            if check_x.is_some() {
                let check_y = check_x.unwrap().get(y + i);
                if check_y.is_some() {
                    let coord = check_y.unwrap();
                    current_string.push(coord.to_owned());
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }
    current_string = String::default();

    for i in 0..XMAS.len() {
        if x >= i && y >= i {
            let check_x = list_2_d.get(x - i);
            if check_x.is_some() {
                let check_y = check_x.unwrap().get(y - i);
                if check_y.is_some() {
                    let coord = check_y.unwrap();
                    current_string.push(coord.to_owned());
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if current_string.eq("XMAS") {
        amount += 1;
    }

    amount
}

pub fn count_xmas() -> i32 {
    let list_2_d: Vec<Vec<char>> = save_input_into_2_d_list();
    let mut sum = 0;
    for i in 0..list_2_d.len() {
        for j in 0..list_2_d.get(i).unwrap().len() {
            sum += horizontal_count(i, j, &list_2_d);
            sum += vertical_count(i, j, &list_2_d);
            sum += diagonal_count(i, j, &list_2_d);
        }
    }
    sum
}
