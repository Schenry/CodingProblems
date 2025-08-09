use std::io::{self, Read};

// Problem - https://leetcode.com/problems/minimum-size-subarray-sum/

///
/// Given an array of positive integers nums and a positive integer target,
/// return the minimal length of a subarray whose sum is greater than or equal to target.
/// If there is no such subarray, return 0 instead.
///
/// Remember that subarray implies contiguous
///
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut count = 0;

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testcase_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn testcase_2() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn testcase_3() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn testcase_4() {
        assert_eq!(min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn testcase_5() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn testcase_6() {
        assert_eq!(
            // vec![83, 28, 26, 25, 25, 25, 25, 25, 12, 12, 4, 2]
            min_sub_array_len(213, vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12]),
            8
        );
    }
}
