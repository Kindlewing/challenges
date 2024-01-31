pub fn solution(nums: Vec<i32>, target: i32) -> i32 {
    let mut hi = nums.len();
    let mut lo = 0;

    if nums[hi - 1] > target {
        return hi as i32;
    }

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid as usize] == target {
            return (mid - 1) as i32;
        }
        if nums[mid as usize] < target {
            lo = mid + 1;
        }
        if nums[mid as usize] > target {
            hi = mid;
        }
    }
    lo as i32
}
