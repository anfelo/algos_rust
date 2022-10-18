use regex::Regex;
// # Directions
//
// Write a function that returns the number of vowels used in a string.
// Vowels are the characters 'a', 'e', 'i', 'o', and 'u'.
//
// # Examples
//
// - vowels("Hi There!") --> 3
// - vowels("Why do you asdk?") --> 4
// - vowels("Why?") --> 0

pub fn vowels(text: String) -> usize {
    let re = Regex::new(r"[aeiouAEIOU]+").unwrap();
    text.chars().filter(|x| re.is_match(&x.to_string())).count()
}

pub fn vowels2(text: String) -> usize {
    let mut count = 0;
    for char in text.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&char) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: usize,
    }

    #[test]
    fn it_counts_vowels() {
        let test_cases = [
            TestCase {
                input: "Hi There!".to_string(),
                expected: 3,
            },
            TestCase {
                input: "Why do you asdk?".to_string(),
                expected: 4,
            },
            TestCase {
                input: "Why".to_string(),
                expected: 0,
            },
        ];

        for test_case in test_cases {
            let result = vowels(test_case.input);

            assert_eq!(result, test_case.expected);
        }
    }
}
