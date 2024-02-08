pub fn solution(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut stock: i32 = 0;
    let mut stock_2: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        if nums[i] == val {
            stock += 1;
        } else {
            stock_2.push(nums[i]);
        }
    } 

    *nums = stock_2;
    println!("{:?}", nums);

    stock
}