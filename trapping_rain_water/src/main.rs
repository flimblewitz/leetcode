fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    println!();
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // my strategy here is going to be to iterate once in both directions
        // each index will be told the biggest thing to its left and the biggest thing to its right
        // then I will iterate one more time to sum up how much water is trapped at each index

        let v: Vec<(i32, i32)> = height
            .into_iter()
            .rev()
            .map(|x| (x, 0))
            .scan(0, |max_to_right, x| {
                let mtr = *max_to_right;
                *max_to_right = (*max_to_right).max(x.0);
                Some((x.0, mtr))
            })
            .collect();

        let v: Vec<(i32, i32, i32)> = v
            .into_iter()
            .rev()
            .map(|x| (x.0, x.1, 0))
            .scan(0, |max_to_left, x| {
                let mtl = *max_to_left;
                *max_to_left = (*max_to_left).max(x.0);
                Some((x.0, x.1, mtl))
            })
            .collect();

        v.into_iter()
            // .inspect(|x| println!("{x:?}"))
            .map(|(h, mtr, mtl)| {
                let max_surrounding_height = mtr.min(mtl);
                if max_surrounding_height > h {
                    max_surrounding_height - h
                } else {
                    0
                }
            })
            .sum()
    }
}

// this works, but it takes too long for the very last test cases
fn _brute_force(mut height: Vec<i32>) -> i32 {
    // let's trim the trailing and leading zeroes
    height = height.into_iter().rev().skip_while(|h| *h == 0).collect();
    height = height.into_iter().rev().skip_while(|h| *h == 0).collect();
    // println!("{height:?}");
    // I don't see a better way to do this than O(n^2), but let's try it
    height
        .iter()
        .enumerate()
        // we don't even need to consider items with value 0
        .filter(|(_, &h)| h > 0)
        // .inspect(|e| println!("i {}, h {}", e.0, e.1))
        .map(|(i, &h)| {
            // for each altitude, I need to find a neighbor to the right
            // if there is a neighbor, I can sum the difference between indexes minus one
            // once I know that altitude and that neighbor's index, then I can
            (1..=h)
                // .inspect(|altitude| println!("altitude {altitude}"))
                .map(|altitude| {
                    (i + 1..height.len())
                        // .inspect(|j| println!("j {j}"))
                        .find(|&j| altitude <= height[j])
                        .and_then(|j| Some(j - i - 1))
                        .unwrap_or(0)
                })
                // .inspect(|horizontal| println!("horizontal: {horizontal}"))
                .sum::<usize>() as i32
        })
        .sum()
}
