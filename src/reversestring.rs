// # Directions
//
// Given a string, return a new string with the reverse order of characters
//
// ## Examples
//
// - reversestring("apple") == "elppa"
// - reversestring("hello") == "olleh"
// - reversestring("Greetings!") == "!sgniteerG"
//
// # Solution 1: (appending to the begining)
//
// ## Pseudocode
//
// - Create an empty string called 'reversed'
// - for each character in provided string
//   - Take the character and add it to the start of 'reversed'
// - Return 'reversed'
pub fn reversestring(s: &String) -> String {
    let mut reverse = String::new();

    for char in s.chars() {
        reverse = char.to_string() + &reverse[..];
    }

    reverse
}

// Using rev + collect
pub fn reversestring1(s: &String) -> String {
    s.chars().rev().collect()
}

// Using loop over rev chars
pub fn reversestring2(s: &String) -> String {
    let mut reverse = String::new();

    for char in s.chars().rev() {
        reverse.push(char);
    }

    reverse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reverses_a_string_option_1() {
        let s = String::from("hello");

        let result = reversestring(&s);

        assert_eq!(result, "olleh");
    }

    #[test]
    fn it_reserses_a_string_option_2() {
        let s = String::from("apple");

        let result = reversestring1(&s);

        assert_eq!(result, "elppa");
    }

    #[test]
    fn it_reserses_a_string_option_3() {
        let s = String::from("Greetings!");

        let result = reversestring2(&s);

        assert_eq!(result, "!sgniteerG");
    }
}
