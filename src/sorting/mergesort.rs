pub fn mergesort(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let middle = left + (right - left) / 2;
        mergesort(nums, left, middle);
        mergesort(nums, middle + 1, right);
        merge(nums, left, middle, right);
    }
}

fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let n1 = middle - left + 1;
    let n2 = right - middle;
    let mut left_nums: Vec<i32> = vec![0; n1];
    let mut right_nums: Vec<i32> = vec![0; n2];

    // copy data into left and right subarrays
    for i in 0..n1 {
        left_nums[i] = nums[left + i];
    }
    for j in 0..n2 {
        right_nums[j] = nums[middle + 1 + j];
    }

    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = left;
    while i < n1 && j < n2 {
        if left_nums[i] <= right_nums[j] {
            nums[k] = left_nums[i];
            i += 1;
        } else {
            nums[k] = right_nums[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        nums[k] = left_nums[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        nums[k] = right_nums[j];
        j += 1;
        k += 1;
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

        let (left, right) = (0, test_case.input.len() - 1);
        mergesort(&mut test_case.input, left, right);

        assert_eq!(test_case.input, test_case.expected);
    }
}
