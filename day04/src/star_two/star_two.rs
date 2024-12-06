use crate::star_one::star_one::save_input_into_2_d_list;

const MAS: &str = "MAS";

fn has_first_diagonal(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> bool {
    let coord = list_2_d.get(x).unwrap().get(y).unwrap();
    let coord_righ_up = list_2_d.get(x + 1);

    let mut coord_right_up_value = char::default();
    let mut coord_left_down_value = char::default();

    if coord_righ_up.is_some() {
        let coord_right_up_value_check = coord_righ_up.unwrap().get(y + 1);

        if coord_right_up_value_check.is_some() {
            coord_right_up_value = coord_right_up_value_check.unwrap().to_owned();
        } else {
            return false;
        }
    } else {
        return false;
    }

    if x >= 1 {
        let coord_left_down = list_2_d.get(x - 1);
        if coord_left_down.is_some() {
            if y >= 1 {
                coord_left_down_value = coord_left_down.unwrap().get(y - 1).unwrap().to_owned();
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    let first_composition = format!(
        "{}{}{}",
        coord_left_down_value,
        coord.to_owned(),
        coord_right_up_value
    );
    let second_composition = format!(
        "{}{}{}",
        coord_right_up_value,
        coord.to_owned(),
        coord_left_down_value
    );

    if first_composition.eq(MAS) || second_composition.eq(MAS) {
        return true;
    }

    false
}

fn has_second_diagonal(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> bool {

    let coord = list_2_d.get(x).unwrap().get(y).unwrap();
    let coord_righ_up = list_2_d.get(x + 1);

    let mut coord_right_up_value = char::default();
    let mut coord_left_down_value = char::default();

    if coord_righ_up.is_some() && y >= 1 {
        let coord_right_up_value_check = coord_righ_up.unwrap().get(y - 1);

        if coord_right_up_value_check.is_some() {
            coord_right_up_value = coord_right_up_value_check.unwrap().to_owned();
        } else {
            return false;
        }
    } else {
        return false;
    }

    if x >= 1 {
        let coord_left_down = list_2_d.get(x - 1);
        if coord_left_down.is_some() {
           
            let coord_left_down_value_check = coord_left_down.unwrap().get(y+1);
            if coord_left_down_value_check.is_some() {
                coord_left_down_value = coord_left_down_value_check.unwrap().to_owned();
            } else  {
                return false;
            }
            
        } else {
            return false;
        }
    } else {
        return false;
    }

    let first_composition = format!(
        "{}{}{}",
        coord_left_down_value,
        coord.to_owned(),
        coord_right_up_value
    );
    let second_composition = format!(
        "{}{}{}",
        coord_right_up_value,
        coord.to_owned(),
        coord_left_down_value
    );

    if first_composition.eq(MAS) || second_composition.eq(MAS) {
        return true;
    }

    false
}


pub fn count_x_formed_mas() -> i32 {
    let list_2_d: Vec<Vec<char>> = save_input_into_2_d_list();
    let mut sum = 0;
    for i in 0..list_2_d.len() {
        for j in 0..list_2_d.get(i).unwrap().len() {
            if has_first_diagonal(i, j, &list_2_d) && has_second_diagonal(i, j, &list_2_d) {
                sum += 1;
            }
        }
    }
    sum
}