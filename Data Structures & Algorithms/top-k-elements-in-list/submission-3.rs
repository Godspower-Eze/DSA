impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts_map: HashMap<i32, i32> = HashMap::new();
        let nums_length: usize = nums.len();
        for j in nums {
            *counts_map.entry(j).or_insert(0) += 1;
        }
        let mut bucket_map: HashMap<i32, Vec<i32>> = (0..=nums_length).rev().map(|k| (k as i32, vec![])).collect();
        // fill the bucket map
        for (k, v) in counts_map {
            bucket_map.get_mut(&v).unwrap().push(k);
        }
        let mut output = vec![];
        for j in (0..=nums_length).rev() {
            let values = bucket_map.get(&(j as i32)).unwrap();
            for v in values {
                output.push(*v);
                if output.len() == (k as usize) {
                    return output;
                }
            }
        }
        return vec![];
    }
}
