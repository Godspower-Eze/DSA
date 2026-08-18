use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums_map: HashMap<i32, i32> = HashMap::new();
        for j in nums {
            *nums_map.entry(j).or_insert(0) += 1;
        }
        let mut sorted: Vec<_> = nums_map.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        let result: Vec<i32> = sorted.into_iter().map(|a| *a.0 ).collect();
        let slice = &result[..(k as usize)];
        slice.into()
    }
}
