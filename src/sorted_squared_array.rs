// # Directions
//
// Write a function that takes in a non-empty array of integers that are sorted in
// ascending order and returns a new array of the same length with the squares of
// the original integers also sorted in ascending order. (watch out for the
// negative numbers!!)
//
// ## Examples
//
// - sorted_squared_array([1, 2, 3, 5, 6, 8, 9]) == [1, 4, 9, 25, 36, 64, 81]

pub fn sorted_squared_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = nums.iter().map(|x| x * x).collect();
    arr.sort_by(|a, b| a.cmp(b));
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        expected: Vec<i32>,
    }

    #[test]
    fn it_returns_sorted_squared_array() {
        let test_cases = [
            TestCase {
                input: vec![1, 2, 3, 5, 6, 8, 9],
                expected: vec![1, 4, 9, 25, 36, 64, 81],
            },
            TestCase {
                input: vec![-4, -3, -2, -1],
                expected: vec![1, 4, 9, 16],
            },
            TestCase {
                input: vec![-4, -3, 3, 5, 6],
                expected: vec![9, 9, 16, 25, 36],
            },
        ];

        for test_case in test_cases {
            let result = sorted_squared_array(test_case.input);

            assert_eq!(result, test_case.expected);
        }
    }
}
