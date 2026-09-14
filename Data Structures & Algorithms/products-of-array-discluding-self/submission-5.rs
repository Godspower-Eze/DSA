impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut prefix = vec![1; len];
        let mut suffix = vec![1; len];
        prefix[0] = nums[0];
        for i in 1..len {
            prefix[i] = nums[i] * prefix[i - 1];
        }
        suffix[len - 1] = nums[len - 1];
        for i in (0..len - 1).rev() {
            suffix[i] = nums[i] * suffix[i + 1];
        }
        let mut output = vec![1; len];
        for i in 0..len {
            if i == 0 {
                output[i] = 1 * suffix[i + 1];
            } else if i == (len - 1) {
                output[i] = prefix[i - 1] * 1;
            } else {
                output[i] = prefix[i - 1] * suffix[i + 1];
            }
        }
        output
    }
}
