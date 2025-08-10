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

/// Okay, so it needs to be contiguous. Pretty obvious this is a sliding window
/// Trick will be the min subarray - we need to consider dropping on the front and expanding on the back
/// Then there could be the case of a less optimal number between a goo number - like
/// [1, 15, 5, 6, 2, 32]
///     ^ this will feel especially large compared to the 2 at the end
///
/// So thinking through the flow - We maintain a window and walk the array, managing the min:
///    1. For each number -
///      a. is the number enough alone? -> return 1
///    2. keep running total - add current num to total
///    3. is running_sum > target?
///      a. yes - this is a valid window - check it against the current min and update if needed
///        1. move left
///      b. no - add right
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // min that satisfies target
    let mut current_min = 0;
    // pointer to left edge of window
    let mut left_index = 0;
    // Current window sum:
    let mut current_sum = 0;

    // Index needed to track left
    for (index, num) in nums.iter().enumerate() {
        // edge case - num alone satisfies:
        if num >= &target {
            return 1; // yes, return - cant do better
        }

        current_sum += num;

        // While loop to shrink the window as we can
        if current_sum >= target {
            while current_sum >= target {
                // Found an answer - update in case it's *the* answer:
                // (right is index+1, left is left_index, current min is right - left)
                let calculated_min = index + 1 - left_index;

                if current_min == 0 || calculated_min < current_min {
                    current_min = calculated_min;
                }

                // if we can lose the left hand digit, drop it and update again
                current_sum -= nums[left_index];
                left_index += 1;
            }
        }
    }

    current_min as i32
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
