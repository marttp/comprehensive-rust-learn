const RADIX: u32 = 10;

pub fn luhn(cc_number: &str) -> bool {
    let validated_cc_number = cc_number.trim();
    if validated_cc_number.is_empty() || validated_cc_number.len() < 2 {
        return false;
    }
    let mut sum: u32 = 0;
    let mut is_double: bool = false;
    let prepared_cc_input = validated_cc_number
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev();
    for c in prepared_cc_input {
        if c.is_alphabetic() {
            return false;
        }
        let digit = c.to_digit(RADIX);
        match digit {
            Some(d) => {
                let final_num = if is_double {
                    sum_digit((d * 2).to_string().as_str())
                } else {
                    d
                };
                sum += final_num;
                is_double ^= true;
            }
            None => return false,
        }
    }
    return sum % 10 == 0;
}

fn sum_digit(string_num: &str) -> u32 {
    let sum: Option<u32> = string_num.chars().map(|c| c.to_digit(RADIX)).sum();
    return sum.unwrap();
}

#[test]
fn test_sum_digit() {
    assert_eq!(5, sum_digit("14"));
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
