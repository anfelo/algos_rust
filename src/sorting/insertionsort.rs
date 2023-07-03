pub fn insertionsort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        // Shift greater items to the right
        while j >= 1 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j = j - 1;
        }

        // Insert item in correct position
        arr[j] = key;
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

        insertionsort(&mut test_case.input);

        assert_eq!(test_case.input, test_case.expected);
    }
}
