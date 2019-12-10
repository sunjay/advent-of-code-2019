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
    let mut found_double = false;
    while num != 0 {
        let digit = num % base;
        num /= base;

        if digit > last_digit {
            return false;

        } else if digit == last_digit {
            found_double = true;
        }
        last_digit = digit;
        digits += 1;
    }

    found_double && digits == 6
}
