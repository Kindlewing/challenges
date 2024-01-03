use std::collections::HashMap;

pub fn solution(nums: Vec<i32>) -> i32 {
    let mut majority = nums[0];
    let mut counter = 0;

    for value in nums {
        if counter == 0 {
            majority = value;
        } else if majority == value {
            counter += 1;
        } else {
            counter -= 1;
        }
    }
    majority
}
