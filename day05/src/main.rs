use day05::star_one::star_one::apply_page_ordering_rules;
use day05::star_two::star_two::sum_sorted_incorrect_updates;

fn main() {
    //all incorrect rules
    let mut incorrect_rules: Vec<Vec<u32>> = vec![];

    //star1
    println!("{}", apply_page_ordering_rules(&mut incorrect_rules));

    let incorrect_rules_immutable = incorrect_rules.clone();

    //star2
    println!(
        "{}",
        sum_sorted_incorrect_updates(&incorrect_rules_immutable)
    );
}
