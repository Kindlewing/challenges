pub fn solution(needle: i32, haystack: Vec<i32>) -> Option<i32> {
    let mut hi: usize = haystack.len();
    let mut lo: usize = 0 as usize;

    while lo < hi {
        let mid = (hi + lo) / 2;
        println!("low: {lo}; \n hi: {hi}; \n mid: {mid};");

        if haystack[mid] == needle {
            return Some(mid as i32);
        } else if haystack[mid] < needle {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    None
}
