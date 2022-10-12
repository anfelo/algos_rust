// # Directions
//
// Given a string, capitalize each of the words of the string
//
// # Examples
//
// - capitalize("hello world") --> "Hello World"

// Not so good - trying to access indexes might not be save
pub fn capitalize_slice(s: &String) -> String {
    let words: Vec<&str> = s.split(" ").collect();
    let mut cap_words: Vec<String> = Vec::new();

    for word in words {
        let cap = word[..1].to_uppercase() + &word[1..];
        cap_words.push(cap);
    }

    cap_words.join(" ")
}

// ASCII save
pub fn capitalize_better(s: &String) -> String {
    let words: Vec<&str> = s.split(" ").collect();
    let mut cap_words: Vec<String> = Vec::new();

    for word in words {
        let mut c = word.chars();
        let cap_word = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        cap_words.push(cap_word);
    }

    cap_words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: String,
    }

    #[test]
    fn it_capitalizes_a_phrase() {
        let test_cases = [
            TestCase {
                input: "hi there, how is it going?".to_string(),
                expected: "Hi There, How Is It Going?".to_string(),
            },
            TestCase {
                input: "i love breakfast at bill miller bbq".to_string(),
                expected: "I Love Breakfast At Bill Miller Bbq".to_string(),
            },
        ];

        for test_case in test_cases {
            let result = capitalize_better(&test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}
