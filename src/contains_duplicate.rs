use std::collections::HashSet;

pub fn solution(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();

    for num in nums {
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }
    false
}
