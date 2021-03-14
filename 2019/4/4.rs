use std::io::Result;

fn main() -> Result<()> {
    let mut password_count: u32 = 0;
    let mut password_count_2: u32 = 0;

    for i in 136818..685979 {
        let _password = i.to_string();
        if is_valid_password(&_password) {
            password_count = password_count + 1;
        }
        if is_valid_password_part_2(&_password) {
            password_count_2 = password_count_2 + 1;
        }
    }

    println!("Valid password count: {}", password_count);
    println!("Valid password count 2: {}", password_count_2);

    Ok(())
}

fn is_valid_password(password: &String) -> bool {
    let mut has_double = false;

    for _j in 0..5 {
        let _password_first_digit = password.chars().nth(_j).unwrap();
        let _password_second_digit = password.chars().nth(_j + 1).unwrap();

        if _password_first_digit.to_digit(10) > _password_second_digit.to_digit(10) {
            return false;
        }

        if _password_first_digit == _password_second_digit {
            has_double = true;
        }
    }

    return has_double;
}

fn is_valid_password_part_2(password: &String) -> bool {
    let mut has_double = false;

    for _j in 0..5 {
        let _password_first_digit = password.chars().nth(_j).unwrap();
        let _password_second_digit = password.chars().nth(_j + 1).unwrap();

        if _password_first_digit.to_digit(10) > _password_second_digit.to_digit(10) {
            return false;
        }

        if _j < 4 {
            let _password_third_digit = password.chars().nth(_j + 2).unwrap();
            if _password_third_digit == _password_second_digit {
                continue;
            }
        }

        if _j > 0 {
            let _password_third_digit = password.chars().nth(_j - 1).unwrap();
            if _password_third_digit == _password_second_digit {
                continue;
            }
        }

        if _password_first_digit == _password_second_digit {
            has_double = true;
        }
    }

    return has_double;
}