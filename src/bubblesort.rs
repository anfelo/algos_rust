pub fn bubblesort(arr: &mut Vec<i32>) {
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
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

        bubblesort(&mut test_case.input);

        assert_eq!(test_case.input, test_case.expected);
    }
}
