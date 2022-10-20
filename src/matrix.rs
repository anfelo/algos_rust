// # Directions
//
// Write a function that accepts an integer N and returns a NxN spiral matrix.
//
// # Examples
//
// - matrix(2)
//
// ```
// [[1, 2],
// [4, 3]]
// ```
//
// - matrix(3)
//
// ```
// [[1, 2, 3],
// [8, 9, 4],
// [7, 6, 5]]
// ```
//
// - matrix(4)
//
// ```
// [[1, 2, 3, 4],
// [12, 13, 14, 5],
// [11, 16, 15, 6],
// [10, 9, 8, 7]]
// ```

pub fn matrix(size: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for _ in 0..size {
        result.push((0..size).collect());
    }

    let mut col_start = 0;
    let mut col_end = size - 1;
    let mut row_start = 0;
    let mut row_end = size - 1;
    let mut count = 1;

    while col_start <= col_end && row_start <= row_end {
        for i in col_start..col_end + 1 {
            result[row_start][i] = count;
            count += 1;
        }
        row_start += 1;
        for i in row_start..row_end + 1 {
            result[i][col_end] = count;
            count += 1;
        }
        col_end -= 1;

        for i in (col_start..col_end + 1).rev() {
            result[row_end][i] = count;
            count += 1;
        }
        row_end -= 1;

        for i in (row_start..row_end + 1).rev() {
            result[i][col_start] = count;
            count += 1;
        }
        col_start += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: usize,
        expected: Vec<Vec<usize>>,
    }

    #[test]
    fn it_returns_nxn_matrix() {
        let test_cases = [
            TestCase {
                input: 2,
                expected: vec![vec![1, 2], vec![4, 3]],
            },
            TestCase {
                input: 3,
                expected: vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            },
            TestCase {
                input: 4,
                expected: vec![
                    vec![1, 2, 3, 4],
                    vec![12, 13, 14, 5],
                    vec![11, 16, 15, 6],
                    vec![10, 9, 8, 7],
                ],
            },
        ];

        for test_case in test_cases {
            let result = matrix(test_case.input);

            assert_eq!(result, test_case.expected);
        }
    }
}
