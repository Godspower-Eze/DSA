impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        let mut result = vec![1; n];

        // left pass: result[i] = product of everything before i
        let mut prefix = 1;
        for i in 0..n {
            result[i] = prefix;
            prefix *= nums[i];
        }

        // right pass: multiply in product of everything after i
        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }

        result
    }
}