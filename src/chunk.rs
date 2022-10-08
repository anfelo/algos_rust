use std::cmp;

// # Directions
//
// Given an array and a chunk size, divide the array into many subarrays where each subarray is of length size
//
// # Examples
//
// - chunk([1,2,3,4], 2) --> [[1,2], [3,4]]
// - chunk([1,2,3,4,5], 2) --> [[1,2], [3,4], [5]]
// - chunk([1,2,3,4,6,7,8], 3) --> [[1,2,3], [4,5,6], [7,8]]
// - chunk([1,2,3,4,5], 4) --> [[1,2,3,4], [5]]
// - chunk([1,2,3,4,5], 10) --> [[1,2,3,4,5]]

pub fn chunk(values: Vec<u8>, size: u8) -> Vec<Vec<u8>> {
    let mut chunks: Vec<Vec<u8>> = Vec::new();

    for i in (0..values.len()).step_by(usize::from(size)) {
        let next_size = cmp::min(i + usize::from(size), values.len());
        let new_chunk = &values[i..next_size];

        chunks.push(new_chunk.to_vec());
    }

    return chunks;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: (Vec<u8>, u8),
        expected: Vec<Vec<u8>>,
    }

    #[test]
    fn it_chunks() {
        let test_cases = [
            TestCase {
                input: (vec![1, 2, 3, 4], 2),
                expected: vec![vec![1, 2], vec![3, 4]],
            },
            TestCase {
                input: (vec![1, 2, 3, 4, 5], 2),
                expected: vec![vec![1, 2], vec![3, 4], vec![5]],
            },
            TestCase {
                input: (vec![1, 2, 3, 4, 5, 6, 7, 8], 3),
                expected: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]],
            },
            TestCase {
                input: (vec![1, 2, 3, 4, 5], 4),
                expected: vec![vec![1, 2, 3, 4], vec![5]],
            },
            TestCase {
                input: (vec![1, 2, 3, 4, 5], 10),
                expected: vec![vec![1, 2, 3, 4, 5]],
            },
        ];

        for test_case in test_cases {
            let result = chunk(test_case.input.0, test_case.input.1);

            assert_eq!(result, test_case.expected)
        }
    }
}
