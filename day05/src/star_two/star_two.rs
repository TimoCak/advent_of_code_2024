pub fn sum_sorted_incorrect_updates(sorted_incorrect_updates: &Vec<Vec<u32>>) -> u32{
    let mut sum = 0;

    for i in 0..sorted_incorrect_updates.len() {
        let sorted_incorrect_update = sorted_incorrect_updates.get(i).unwrap();
        sum += sorted_incorrect_update.get(sorted_incorrect_update.len() / 2).unwrap();
    }
    println!("{}", sorted_incorrect_updates.len());
    sum
}