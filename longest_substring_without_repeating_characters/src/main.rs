fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    assert_eq!(Solution::length_of_longest_substring(" ".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("abba".into()), 2);
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // we'll set a left index for the current substring
        let mut i: usize = 0;
        // we'll remember the indexes where we saw each char
        let mut observed_char_indexes: HashMap<char, usize> = HashMap::new();
        // on each iteration, we'll reassess the max observed length of a char-unique substring
        let mut max_substring_length: usize = 0;
        for (j, c) in s.chars().enumerate() {
            // if we encounter a duplicate char, let's reset the current substring to start at the index immediately following that char's previous occurrence. We'll also need to scrap the observed indexes for every char preceding it in the substring
            if let Some(&index_of_previous_char_occurrence) = observed_char_indexes.get(&c) {
                s.get(i..index_of_previous_char_occurrence + 1)
                    .unwrap()
                    .chars()
                    .for_each(|c| {
                        observed_char_indexes.remove(&c);
                    });
                i = index_of_previous_char_occurrence + 1;
            }
            // we know that the substring s[i..=j] is char-unique
            // let's record that we saw this char here at index j and then reassess the max length
            observed_char_indexes.insert(c, j);
            max_substring_length = max_substring_length.max(j - i + 1);
        }
        max_substring_length as i32
    }
}
