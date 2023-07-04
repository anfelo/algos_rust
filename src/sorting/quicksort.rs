pub fn quicksort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if high > low {
        let pivot = partition(arr, low, high);
        quicksort(arr, low, pivot - 1);
        quicksort(arr, pivot + 1, high);
    }
}

pub fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot_item = arr[low];
    let mut left = low;
    let mut right = high;

    while left < right {
        while arr[left] <= pivot_item && left < high {
            left += 1;
        }

        while arr[right] > pivot_item && right > low {
            right -= 1;
        }

        if left < right {
            arr.swap(left, right);
        }
    }

    arr[low] = arr[right];
    arr[right] = pivot_item;
    return right;
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

        let (low, high) = (0, test_case.input.len() - 1);
        quicksort(&mut test_case.input, low, high);

        assert_eq!(test_case.input, test_case.expected);
    }
}
