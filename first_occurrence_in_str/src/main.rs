fn main() {
    // println!("{:?}", Solution::str_str("sadbutsad".into(), "sad".into()));
    // println!("{:?}", Solution::str_str("leetcode".into(), "leeto".into()));
    println!("{:?}", Solution::str_str("abb".into(), "abaaa".into()));
}

struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let first_index_occurrence = -1;
        let h_len = haystack.len();
        let n_len = needle.len();
        if n_len > h_len {
            return first_index_occurrence;
        }
        for i in 0..(h_len - n_len + 1) {
            if haystack.chars().skip(i).take(n_len).collect::<String>() == needle {
                return i as i32;
            }
        }
        first_index_occurrence
    }
}
