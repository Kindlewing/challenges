pub fn solutiion(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();

    for kid in &candies {
        if kid + extra_candies >= *candies.iter().max().unwrap() {
            result.push(true);
        } else {
            result.push(false);
        }
    }

    result
}
