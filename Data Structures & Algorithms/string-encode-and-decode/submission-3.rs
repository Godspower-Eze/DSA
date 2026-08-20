impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("empty");
        }
        strs.join("-1")
    }

    pub fn decode(s: String) -> Vec<String> {
        if s == String::from("") {
            return vec![String::from("")];
        } else if s == String::from("empty") {
            return vec![];
        }
        s.split("-1").map(|x| x.to_string()).collect()
    }
}
