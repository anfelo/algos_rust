pub fn selectionsort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        let mut max = 0;
        for j in 0..arr.len() - i {
            if arr[j] > arr[max] {
                max = j;
            }
        }

        let last_idx = arr.len() - 1 - i;
        if max != last_idx {
            arr.swap(max, last_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        expected: Vec<i32>,
    }

    #[test]
    fn it_sorts_the_input_array() {
        let mut test_case = TestCase {
            input: vec![10, 4, 43, 5, 57, 91, 45, 9, 7],
            expected: vec![4, 5, 7, 9, 10, 43, 45, 57, 91],
        };

        selectionsort(&mut test_case.input);

        assert_eq!(test_case.input, test_case.expected);
    }
}
