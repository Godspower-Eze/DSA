use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut store = HashSet::with_capacity(nums.len());
        for n in nums {
            if !store.insert(n) {
                return true
            }
        }
        return false
    }
}
