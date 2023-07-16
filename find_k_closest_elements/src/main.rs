fn main() {
    assert_eq!(
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![-2, -1, 1, 2, 3, 4, 5], 7, 3),
        vec![-2, -1, 1, 2, 3, 4, 5]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![0, 0, 1, 2, 3, 3, 4, 7, 7, 8], 3, 5),
        vec![3, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![1, 1, 1, 10, 10, 10], 1, 9),
        vec![10]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![3, 5, 8, 10], 2, 15),
        vec![8, 10]
    );
    assert_eq!(Solution::find_closest_elements(vec![1, 3], 1, 2), vec![1]);
}
struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // so first I'll find the element that is closest to x, which may itself be x
        let closest_index = match arr.binary_search(&x) {
            Ok(i) => i,
            Err(i) => {
                // i is where x would be inserted

                // i might be out of bounds. If so, i-1 is the closest index
                if i == arr.len() {
                    i - 1
                }
                // otherwise, the current resident of i is greater than x
                // we need to compare the residents of i-1 and i and pick the index of the one that's closest to x. Ties are won by the smaller one
                else if i > 0 {
                    match (x - arr[i - 1]).cmp(&(arr[i] - x)) {
                        std::cmp::Ordering::Less => i - 1,
                        std::cmp::Ordering::Equal => {
                            *[i - 1, i].iter().min_by_key(|index| arr[**index]).unwrap()
                        }
                        std::cmp::Ordering::Greater => i,
                    }
                } else {
                    i
                }
            }
        };

        // and now I crawl outward to find the k-1 closest elements
        let mut l = closest_index;
        let mut r = closest_index;
        // as a reminder, the number of elements in a range [l, r] is (r-l+1)
        // I could have made a closure or something for that expression, but whatever
        while r - l + 1 < k as usize {
            match (
                l.checked_sub(1),
                if r + 1 < arr.len() { Some(r + 1) } else { None },
            ) {
                (None, None) => unreachable!("this would mean that k exceeds the array length"),
                (None, Some(_)) => r += k as usize - (r - l + 1),
                (Some(_), None) => l -= k as usize - (r - l + 1),
                (Some(next_l), Some(next_r)) => match (x - arr[next_l]).cmp(&(arr[next_r] - x)) {
                    std::cmp::Ordering::Less => l = next_l,
                    std::cmp::Ordering::Equal => {
                        if arr[next_l] < arr[next_r] {
                            l = next_l
                        } else {
                            r = next_r
                        }
                    }
                    std::cmp::Ordering::Greater => r = next_r,
                },
            }
        }

        arr.get(l..=r).unwrap().into()
    }
}
