use std::fs;

pub fn apply_page_ordering_rules(incorrect_updates: &mut Vec<Vec<u32>>) -> u32 {
    let rules = get_rules();
    let updates = get_updates();
    let mut sum: u32 = 0;
    for i in updates.clone() {
        let mut rule_applied = true;
        let mut sort_incorrect_update = i.clone();
        for j in i.clone() {
            let rules_for_el: Vec<(u32, u32)> = get_all_rules_for_el(&rules, j);
            for rule in rules_for_el {
                if !is_before(&i, rule.0, rule.1) {
                    rule_applied = false;
                    break;
                    //move to another function
                    /*let sort_incorrect_update_immutable = sort_incorrect_update.clone();
                    let index1 = get_index(&sort_incorrect_update_immutable, rule.0);
                    let index2 = get_index(&sort_incorrect_update_immutable, rule.1);

                    if index1 < sort_incorrect_update_immutable.len()
                        && index2 < sort_incorrect_update_immutable.len()
                    {
                        sort_incorrect_update.swap(index1, index2);
                    } */
                }
            }
        }
        if rule_applied {
            sum += i.get(i.len() / 2).unwrap();
        } else {
            incorrect_updates.push(sort_incorrect_update);
        }
    }
    sum
}

fn get_rules() -> Vec<(u32, u32)> {
    let mut input_string = fs::read_to_string("./input.txt").expect("should read input file");
    input_string = input_string.replace("\r", "");
    let mut current_number_string = String::default();
    let input_string_splitted: Vec<&str> = input_string.split("\n\n").collect();
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut current_tuple = (0, 0);

    let mut rules_string = input_string_splitted.get(0).unwrap().to_owned().to_owned();

    rules_string.push('\n');

    for c in rules_string.chars() {
        if c == '|' {
            current_tuple.0 = current_number_string.parse::<u32>().unwrap();
            current_number_string = String::default();
        } else if c == '\n' {
            current_tuple.1 = current_number_string.parse::<u32>().unwrap();
            rules.push(current_tuple);
            current_number_string = String::default();
        } else {
            current_number_string.push(c);
        }
    }
    rules
}

fn get_updates() -> Vec<Vec<u32>> {
    let mut input_string = fs::read_to_string("./input.txt").expect("should read input file");
    input_string = input_string.replace("\r", "");
    let mut current_number_string = String::default();
    let input_string_splitted: Vec<&str> = input_string.split("\n\n").collect();
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut current_list: Vec<u32> = vec![];

    let updates_string = input_string_splitted.get(1).unwrap().to_owned().to_owned();

    for c in updates_string.chars() {
        if c == ',' {
            current_list.push(current_number_string.parse::<u32>().unwrap());
            current_number_string = String::default();
        } else if c == '\n' {
            current_list.push(current_number_string.parse::<u32>().unwrap());
            updates.push(current_list.clone());
            current_number_string = String::default();
            current_list = vec![];
        } else {
            current_number_string.push(c);
        }
    }
    updates
}

fn is_before(list: &Vec<u32>, el1: u32, el2: u32) -> bool {
    let mut first = false;
    for i in list {
        if el1 == i.to_owned() {
            first = true;
            break;
        }
        if el2 == i.to_owned() {
            first = false;
            break;
        }
    }
    first
}

fn get_all_rules_for_el(rules: &Vec<(u32, u32)>, el: u32) -> Vec<(u32, u32)> {
    let mut rules_for_el: Vec<(u32, u32)> = vec![];

    for rule in rules {
        if rule.0.to_owned() == el {
            rules_for_el.push(rule.to_owned());
        }
    }
    rules_for_el
}

fn get_index(list: &Vec<u32>, el: u32) -> usize {
    for i in 0..list.len() {
        if list.get(i).unwrap().to_owned() == el {
            return i;
        }
    }
    list.len()
}
