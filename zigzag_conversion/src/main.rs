fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".into(), 4));
}

struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let chars: Vec<char> = s.chars().collect();
        // chars;
        let mut rows: Vec<String> = vec!["".into(); num_rows as usize];
        s.chars().for_each(|c| {});
        let mut chars_index = 0;
        while chars_index < chars.len() {
            // zig
            // println!("zig");
            for row_number in 0..num_rows {
                if chars_index >= chars.len() {
                    break;
                }
                // println!("{}, {row_number}", chars[chars_index]);
                rows[row_number as usize].push(chars[chars_index]);
                chars_index += 1;
            }
            // zag
            // println!("zag");
            for row_number in (1..(num_rows - 1)).rev() {
                if chars_index >= chars.len() {
                    break;
                }
                // println!("{}, {row_number}", chars[chars_index]);
                rows[row_number as usize].push(chars[chars_index]);
                chars_index += 1;
            }
        }
        // println!("{:?}", rows);
        rows.join("")
    }
}
