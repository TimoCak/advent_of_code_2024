use day01::utils::file::read_input_file;

fn main() {
    //Excercise1 results:
    println!("{}", calc_distance());

    //Excercise2 results:
    println!("{}", calc_similarity_scores());
}

fn save_input_as_lists() -> (Vec<i32>, Vec<i32>) {
    let input = read_input_file("./input.txt");
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    let mut current_word: String = String::default();

    for c in input.chars() {
        
        if c == ' ' {
            if !current_word.is_empty() {
                list1.push(current_word.trim().parse::<i32>().unwrap());
            }
            current_word = String::default();
        } else if c == '\n' {
            if !current_word.is_empty() {
                list2.push(current_word.trim().parse::<i32>().unwrap());
            }
            current_word = String::default();
        }
         else if !String::from(c).is_empty() {
            current_word.push(c);
        }
        
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

fn calc_distance() -> i32 {
    let list_tuple = save_input_as_lists();
    let mut distance_sum: i32 = 0;
    for i in 0..list_tuple.0.len() {
        distance_sum += i32::abs(list_tuple.0.get(i).unwrap() - list_tuple.1.get(i).unwrap());
    }
    distance_sum
}

fn calc_similarity_scores() -> i32 {
    let mut similarity_scores: i32 = 0;
    let list_tuple = save_input_as_lists();

    for list1_item in &list_tuple.0 {
        let mut similarity_counter = 0;
        for list2_item in &list_tuple.1 {
            if list1_item == list2_item {
                
                similarity_counter = similarity_counter + 1;
            }
        }
        //push list
        let similarity_score = list1_item.to_owned() * similarity_counter;
        similarity_scores += similarity_score;
    } 
    
    similarity_scores    
}