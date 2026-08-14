impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut store = vec![];
        for n in nums {
            if store.contains(&n) {
                return true
            }
            store.push(n)
        }
        return false
    }
}
