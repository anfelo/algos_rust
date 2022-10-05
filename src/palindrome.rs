// # Directions
// Given a string, return true if a string is a palindrome or false if it is not. Palindromes are strings that form the same word when reversed. Do include spaces and punctuation in determining is the string is a palindrome.
//
// ## Examples
//
// - palindrome("abba") == true
// - palindrome("abcdefg") == false

pub fn palindrome(s: &String) -> bool {
    let reverse: String = s.chars().rev().collect();

    &reverse == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_palindrome() {
        let s = String::from("abba");

        let is_palindrome = palindrome(&s);

        assert!(is_palindrome);
    }

    #[test]
    fn it_is_not_palindrome() {
        let s = String::from("abcdefg");

        let is_palindrome = palindrome(&s);

        assert!(!is_palindrome);
    }
}
