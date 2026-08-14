impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_vec: Vec<char> = s.chars().collect();
        s_vec.sort();
        let mut t_vec: Vec<char> = t.chars().collect();
        t_vec.sort();
        (s_vec == t_vec)
    }
}
