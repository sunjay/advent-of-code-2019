mod digits;
mod dedup;

use digits::GetDigits;
use dedup::DedupCount;

fn main() {
    let input_range = 356261..=846303;

    let valid_passwords = input_range.filter(|&p| is_valid_password(p)).count();
    println!("{}", valid_passwords);
}

fn is_valid_password(password: u64) -> bool {
    let digits = password.digits(10);
    let digits_len = digits.len();

    let grouped_digits = digits.dedup_count();

    let found_matching_digits = grouped_digits.clone().any(|(_, count)| count == 2);

    let mut never_decrease = true;
    // Digits can range from 0 to 9, so 10 is always greater
    let mut last_digit = 10;
    for (digit, _) in grouped_digits {
        if digit > last_digit {
            never_decrease = false;
        }
        last_digit = digit;
    }

    digits_len == 6 && found_matching_digits && never_decrease
}
