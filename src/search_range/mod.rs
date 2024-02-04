pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first: i32 = -1;
    let mut last: i32 = -1;

    for (i, &num) in nums.iter().enumerate() {
        if num == target {
            first = i as i32;
            break;
        }
    }

    for (i, &num) in nums.iter().enumerate().rev() {
        if num == target {
            last = i as i32;
            break;
        }
    }

    vec![first, last]
}
