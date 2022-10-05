// # Directions
//
// Given an integer, return an integer that is the reverse ordering of numbers
//
// ## Examples
//
// - reverse(15) == 51
// - reverse(981) == 189
// - reverse(500) == 5
// - reverse(-15) == -51
// - reverse(-90) == -90

pub fn reverseint(num: i8) -> i8 {
    let sign = if num < 0 { -1 } else { 1 };
    let mut num_str = num.to_string();

    if sign < 0 {
        num_str = String::from(&num_str[1..]);
    }

    let reverse_num_str: String = num_str.chars().rev().collect();

    reverse_num_str.parse::<i8>().unwrap() * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reverses_an_int() {
        let reverse_num = reverseint(15);

        assert_eq!(reverse_num, 51);
    }

    #[test]
    fn it_reverses_a_negative_int() {
        let reverse_num = reverseint(-15);

        assert_eq!(reverse_num, -51)
    }
}
