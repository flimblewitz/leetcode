fn main() {
    assert_eq!(
        Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
        'c'
    );
    assert_eq!(
        Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
        'f'
    );
    assert_eq!(
        Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'),
        'x'
    );
    assert_eq!(
        Solution::next_greatest_letter(vec!['e', 'e', 'g', 'g'], 'g'),
        'e'
    );
}

struct Solution {}
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l = 0;
        let mut r = letters.len() - 1;
        while l < r {
            // normally, we're looking for specifically the target
            // that would look like this
            let m = (l + r) / 2;
            if target <= letters[m] {
                r = m;
            } else {
                l = m + 1;
            }
        }

        // it's possible that nums[r] is part of a stretch of duplicate letters
        // let's move r to the last index of those duplicate letters
        // this forces r to become either the index of the first letter greater than the target OR the last index
        while target == letters[r] && r < letters.len() - 1 {
            r += 1;
        }

        // println!("letters: {letters:?}");
        // println!("target: {target}");
        // println!("r: {r}");

        // so either nums[r] is the first letter greater than the target, or we have to use the fallback answer to indicate we had no such letter
        if target >= letters[r] {
            letters[0]
        } else {
            letters[r]
        }
    }
}
