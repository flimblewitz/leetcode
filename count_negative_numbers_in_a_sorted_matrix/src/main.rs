fn main() {
    assert_eq!(
        Solution::count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ]),
        8
    );
    assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
}

struct Solution {}
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        // one optimization is to look at the bottom right. If it's >=0, we know the answer is 0
        // another optimization would be to do all of this in parallel for each row

        // all the other lists I've seen so far have been non-decreasing order. So I can flip my logic or flip the vecs. I think I'll flip my logic

        // one way to do this is to look at each row and search for... -1
        // I know that r will end up being the first occurrence of -1, if there is one
        // then I'll know how many negatives are in the row based on whether nums[r] is negative and the row length

        // then I repeat for each row

        let mut sum = 0;
        for row in grid {
            // println!("row {row:?}");
            let mut l = 0;
            let mut r = row.len() - 1;
            while l < r {
                let m = (l + r) / 2;
                if -1 >= row[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            // println!("r {r}");
            sum += if row[r] < 0 { row.len() - r } else { 0 }
        }

        sum as i32
    }
}
