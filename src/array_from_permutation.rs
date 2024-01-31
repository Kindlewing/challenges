pub fn solution(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        output.push(nums[nums[i] as usize]);
    }

    output
}
