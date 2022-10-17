// # Directions
//
// Write a function that accepts a positive number N. The function should console log a pyramid shape with N levels using the # character. Make sure the pyramid has spaces on both the left _and_ right hand sides
//
// # Examples
//
// - pyramid(1)
//
// ```
// "#"
// ```
//
// - pyramid(2)
//
// ```
// " # "
// "###"
// ```
//
// - pyramid(3)
//
// ```
// "  #  "
// " ### "
// "#####"
// ```
//
// - pyramid(4)
//
// ```
// "   #   "
// "  ###  "
// " ##### "
// "#######"
// ```
pub fn pyramid(num: u8) -> String {
    let mut result = String::from("");
    let mut extra_step = 0;

    for i in 1..num + 1 {
        let step = "#".repeat(usize::from(i) + extra_step);
        let spaces = " ".repeat(usize::from(num - i));

        result.push_str(&format!("{}{}{}", spaces, step, spaces));

        if i < num {
            result.push_str("\n");
        }

        extra_step += 1;
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
                expected: " # \n###".to_string(),
            },
            TestCase {
                input: 3,
                expected: "  #  \n ### \n#####".to_string(),
            },
            TestCase {
                input: 4,
                expected: "   #   \n  ###  \n ##### \n#######".to_string(),
            },
        ];

        for test_case in test_cases {
            let result = pyramid(test_case.input);

            assert_eq!(result, test_case.expected)
        }
    }
}
