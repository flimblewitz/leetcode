fn main() {
    println!("{:?}", Solution::generate(1));
    println!("{:?}", Solution::generate(2));
    println!("{:?}", Solution::generate(5));
}

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut v = vec![vec![1]];
        (1..num_rows as usize).for_each(|row_index| {
            // println!("row: {row}");
            let prev_row = &v[row_index - 1];
            // println!("prev_row: {prev_row:?}");
            v.push(
                // (0..prev_row.len())
                (0..=row_index)
                    .map(|i| {
                        i.checked_sub(1)
                            .and_then(|prev_i| Some(prev_row[prev_i]))
                            .unwrap_or(0)
                            + prev_row.get(i).unwrap_or(&0)
                    })
                    .collect(),
            );
        });
        v
    }
}
