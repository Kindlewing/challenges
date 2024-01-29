pub fn solution(input: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..input.len() - 1 {
        for j in 0..input.len() - 1 - i {
            if input[j] > input[j + 1] {
                let tmp = input[j];
                input[j] = input[j + 1];
                input[j + 1] = tmp;
            }
        }
    }
    input.to_vec()
}
