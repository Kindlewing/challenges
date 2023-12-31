use std::collections::hash_map::HashMap;

pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, element) in nums.iter().enumerate() {
        match map.get(element) {
            Some(val) => return vec![*val, i as i32],
            None => map.insert(target - element, i as i32),
        };
    }
    vec![]
}
