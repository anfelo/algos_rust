use regex::Regex;

// # Directions
//
// Check to see if two provided strings are anagrams of each other.
// One string is an anagram of another if it uses the same characters in the
// same quantity. Only consider characters, not spaces or punctuation.
// Consider capital letters to be the same as lower case.
//
// # Examples
//
// - anagrams("rail safety", "fairy tales") --> true
// - anagrmas("RAIL! SAFETY!", "fairy tales") --> true
// - anagrams("Hi there!", "Bye there") --> false

pub fn anagrams(string1: String, string2: String) -> bool {
    let re = Regex::new(r"\W+").unwrap();
    let clean_str1 = re.replace_all(&string1, "").to_lowercase();
    let clean_str2 = re.replace_all(&string2, "").to_lowercase();

    if clean_str1.len() != clean_str2.len() {
        return false;
    }

    let mut chars1: Vec<char> = clean_str1.chars().collect();
    let mut chars2: Vec<char> = clean_str2.chars().collect();

    chars1.sort_by(|a, b| b.cmp(a));
    chars2.sort_by(|a, b| b.cmp(a));

    String::from_iter(chars1) == String::from_iter(chars2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: (String, String),
        expected: bool,
    }

    #[test]
    fn it_should_be_anagrams() {
        let test_cases = [
            TestCase {
                input: ("rail safety".to_string(), "fairy tales".to_string()),
                expected: true,
            },
            TestCase {
                input: ("RAIL! SAFETY!".to_string(), "fairy tales".to_string()),
                expected: true,
            },
            TestCase {
                input: ("Hi there!".to_string(), "Bye there".to_string()),
                expected: false,
            },
        ];

        for test_case in test_cases {
            let result = anagrams(test_case.input.0, test_case.input.1);
            assert_eq!(result, test_case.expected);
        }
    }
}
