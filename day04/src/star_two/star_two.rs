use crate::star_one::star_one::save_input_into_2_d_list;

const MAS: &str = "MAS";

fn has_first_diagonal(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> bool {
    let coord = list_2_d.get(x).unwrap().get(y).unwrap();
    let coord_x = list_2_d.get(x + 1);

    let mut coord_x_value = char::default();
    let mut coord_y_value = char::default();

    if coord_x.is_some() {
        let coord_x_value_check = coord_x.unwrap().get(y + 1);

        if coord_x_value_check.is_some() {
            coord_x_value = coord_x_value_check.unwrap().to_owned();
        } else {
            return false;
        }
    } else {
        return false;
    }

    if x >= 1 {
        let coord_y = list_2_d.get(x - 1);
        if coord_y.is_some() {
            if y >= 1 {
                coord_y_value = coord_y.unwrap().get(y - 1).unwrap().to_owned();
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    let first_composition = format!("{}{}{}", coord_y_value, coord.to_owned(), coord_x_value);
    let second_composition = format!("{}{}{}", coord_x_value, coord.to_owned(), coord_y_value);

    if first_composition.eq(MAS) || second_composition.eq(MAS) {
        return true;
    }

    false
}

fn has_second_diagonal(x: usize, y: usize, list_2_d: &Vec<Vec<char>>) -> bool {
    let coord = list_2_d.get(x).unwrap().get(y).unwrap();
    let coord_x = list_2_d.get(x + 1);

    let mut coord_x_value = char::default();
    let mut coord_y_value = char::default();

    if coord_x.is_some() && y >= 1 {
        let coord_x_value_check = coord_x.unwrap().get(y - 1);

        if coord_x_value_check.is_some() {
            coord_x_value = coord_x_value_check.unwrap().to_owned();
        } else {
            return false;
        }
    } else {
        return false;
    }

    if x >= 1 {
        let coord_y = list_2_d.get(x - 1);
        if coord_y.is_some() {
            let coord_y_value_check = coord_y.unwrap().get(y + 1);
            if coord_y_value_check.is_some() {
                coord_y_value = coord_y_value_check.unwrap().to_owned();
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

    let first_composition = format!("{}{}{}", coord_y_value, coord.to_owned(), coord_x_value);
    let second_composition = format!("{}{}{}", coord_x_value, coord.to_owned(), coord_y_value);

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
