pub fn solution(nums: Vec<i32>) -> bool {
    let mut gate: bool = false;

    for i in 0..nums.len() - 1 {
        if nums[i] <= nums[i+1] {
            gate = true;
        } else {
            gate = false;
            break;
        }
    }

    if gate { return gate }

    for i in 0..nums.len() - 1 {
        if nums[i] >= nums[i+1] {
            gate = true;
        } else {
            gate = false;
            break;
        }
    }

    gate
}
