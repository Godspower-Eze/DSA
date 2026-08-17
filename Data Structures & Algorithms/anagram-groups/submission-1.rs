const NUM_OF_LETTERS: usize = 26;

fn create_key(x: &String) -> [i32; NUM_OF_LETTERS] {
    let mut counts = [0i32; 26];
    let byte_of_char_a = b'a';
    for a in x.bytes() {
        counts[(a - byte_of_char_a) as usize] += 1;
    }
    counts
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[i32; NUM_OF_LETTERS], Vec<String>> = HashMap::new();

        for s in strs.into_iter() {
            let key = create_key(&s);

            groups.entry(key).or_insert_with(Vec::new).push(s);
        }

        groups.into_values().collect()
    }
}
