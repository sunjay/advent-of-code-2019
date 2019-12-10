fn main() {
    let input_range = 356261..=846303;

    let valid_passwords = input_range.filter(|&p| is_valid_password(p)).count();
    println!("{}", valid_passwords);
}

fn is_valid_password(password: u64) -> bool {
    // If each digit from left to right can never decrease, then each digit from right to left can
    // only decrease or stay the same.

    let mut num = password;
    let base = 10;
    // Digits can range from 0 to 9, so 10 is always greater
    let mut last_digit = 10;
    let mut digits = 0;
    let mut matching_count = None;
    let mut found_any_pair = false;
    while num != 0 {
        let digit = num % base;
        num /= base;

        if digit > last_digit {
            return false;

        } else if digit == last_digit {
            match matching_count {
                Some((matching_digit, count)) if matching_digit != digit => {
                    if count == 2 {
                        found_any_pair = true;
                    }
                    // Last digit + current digit = 2 digits
                    matching_count = Some((digit, 2));
                },
                Some((matching_digit, count)) => {
                    let count = count + 1;
                    // Groups greater than 2 don't count
                    if count > 2 {
                        matching_count = None;

                    } else {
                        matching_count = Some((matching_digit, count));
                    }
                },
                None => {
                    // Last digit + current digit = 2 digits
                    matching_count = Some((digit, 2));
                },
            }
        }

        last_digit = digit;
        digits += 1;
    }

    if !found_any_pair {
        if let Some((_, 2)) = matching_count {
            found_any_pair = true;
        }
    }

    found_any_pair && digits == 6
}
