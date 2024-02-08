pub fn solution(nums: &mut Vec<i32>, k: i32) -> () {
    for _ in 0..k {
        nums.insert(0, nums[nums.len()-1]);
        nums.pop();
    }
}