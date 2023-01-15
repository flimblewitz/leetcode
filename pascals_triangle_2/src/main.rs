fn main() {
    println!("{:?}", Solution::get_row(0));
    println!("{:?}", Solution::get_row(1));
    println!("{:?}", Solution::get_row(2));
    println!("{:?}", Solution::get_row(3));
    println!("{:?}", Solution::get_row(5));
}

struct Solution;

/*
0    1
1    1 1
2    1 2 1
3    1 3 3 1
4    1 4 6 4 1
5    1 5 10 10 5 1
6    1 6 15 20 15 6 1
7    1 7 21 35 35 21 7 1
8    1 8 28 56 70 56 28 8 1
9    1 9 36 84 126 126 84 36 9 1
10   1 10 45 120 210 252 210 120 45 10 1
*/

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];
        (0..=(row_index as usize)).for_each(|row_index| {
            row = (0..=row_index)
                .map(|i| {
                    i.checked_sub(1).and_then(|i| Some(row[i])).unwrap_or(0)
                        + row.get(i).unwrap_or(&0)
                })
                .collect();
        });
        row
    }
}
