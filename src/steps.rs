// # Directions
//
// Write a function that accepts a positive number N. The function should console log a step shape with N levels using the # character. Make sure the step has spaces on the right hand side! (str repeat).
//
// # Examples
//
// - steps(2)
//
// ```
// "# "
// "##"
// ```
//
// - steps(3)
//
// ```
// "#  "
// "## "
// "###"
// ```
//
// - steps(4)
//
// ```
// "#   "
// "##  "
// "### "
// "####"
// ```
pub fn steps(num: u8) -> String {
    let mut result = String::from("");

    for i in 1..num + 1 {
        for j in 1..num + 1 {
            if j > i {
                result.push_str(" ");
            } else {
                result.push_str("#");
            }
        }

        if i < num {
            result.push_str("\n")
        }
    }

    result
}

pub fn steps_repeat(num: u8) -> String {
    let mut result = String::from("");

    for i in 1..num + 1 {
        let step = "#".repeat(usize::from(i));
        let spaces = " ".repeat(usize::from(num - i));
        result.push_str(&format!("{}{}", step, spaces));

        if i < num {
            result.push_str("\n")
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: u8,
        expected: String,
    }

    #[test]
    fn it_prints_steps() {
        let test_cases = [
            TestCase {
                input: 1,
                expected: "#".to_string(),
            },
            TestCase {
                input: 2,
                expected: "# \n##".to_string(),
            },
            TestCase {
                input: 3,
                expected: "#  \n## \n###".to_string(),
            },
            TestCase {
                input: 4,
                expected: "#   \n##  \n### \n####".to_string(),
            },
        ];

        for test_case in test_cases {
            let result = steps_repeat(test_case.input);

            assert_eq!(result, test_case.expected)
        }
    }
}
