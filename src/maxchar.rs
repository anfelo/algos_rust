use std::collections::HashMap;

// # Directions
//
// Given a string, return the character that is most commonly used in the string.
//
// ## Examples
//
// - maxchar("abbaa") == "a"
// - maxchar("abcdefg 12311") == "1"
//
// ## Common String Questions
//
// - What is the most common character in the string?
// - Does string A have the same characters as string B (is it an anagram)?
// - Does the given string have any repeated characters in it?

pub fn maxchar(s: &String) -> String {
    let mut chars = HashMap::new();
    let mut max = (String::from(""), 0);

    for char in s.chars() {
        let count = chars.entry(char.to_string()).or_insert(0);
        *count += 1;

        if *count > max.1 {
            max = (char.to_string(), *count);
        }
    }

    return max.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_maxchars() {
        let s = String::from("abbaa");

        let result = maxchar(&s);

        assert_eq!(result, "a");
    }
}
