use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
    );
    println!("{:?}", Solution::largest_rectangle_area(vec![1]));
    println!("{:?}", Solution::largest_rectangle_area(vec![0, 9]));
}
struct Solution {}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut sub_solutions: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();
        let _ = recursively_get_rectangles_for_each_bar(&mut sub_solutions, &heights, 0);
        sub_solutions
            .values()
            .map(|rectangles| {
                rectangles
                    .iter()
                    .map(|(height, width)| height * width)
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}
// this was not a performant solution
fn recursively_get_rectangles_for_each_bar(
    sub_solutions: &mut HashMap<usize, Vec<(i32, i32)>>,
    heights: &[i32],
    index: usize,
) -> Vec<(i32, i32)> {
    /*
    for every bar, its best rectangle going right is one of
    - itself
    - any of the rectangles formed by matching the height of any lower subsequent "unmasked" bar that is not preceded by a shorter bar
    so every bar has n rectangles it can form. Any of these might end up being the right hand side of a winning rectangle
    */
    // if we already solved this subproblem, reuse the solution
    if let Some(rectangles) = sub_solutions.get(&index) {
        return rectangles.clone();
    }
    // if this is the last bar, its list of best rectangles is comprised of its own height and width
    if index == heights.len() - 1 {
        let v = vec![(heights[index], 1)];
        sub_solutions.insert(index, v.clone());
        return v;
    }
    // if we haven't solved this one yet and it's not an edge case, then we have to "merge" with the next bar's rectangles
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let rectangles_for_next_bar =
        recursively_get_rectangles_for_each_bar(sub_solutions, heights, index + 1);
    // we need to stick the current bar in this hashmap because it is a rectangle to consider as well
    hm.insert(heights[index], 1);
    for (height, width) in rectangles_for_next_bar {
        if height <= heights[index] {
            hm.insert(height, width + 1);
        } else {
            hm.entry(heights[index])
                .and_modify(|existing_entry_width| {
                    *existing_entry_width = (*existing_entry_width).max(width + 1)
                })
                .or_insert(width + 1);
        }
    }
    let v: Vec<(i32, i32)> = hm.into_iter().collect();
    sub_solutions.insert(index, v.clone());
    v
}

// the problem is that O(n^2) doesn't hold up
fn _simple(heights: Vec<i32>) -> i32 {
    // I think I want to iterate over pairs of start and end
    // each stretch of start to end will track a minimum height of bars between
    // max will be repeatedly compared to (end-start)*(min height seen for given start)
    let mut max = 0;
    for start in 0..heights.len() {
        let mut min_height = heights[start];
        for end in start..heights.len() {
            min_height = min_height.min(heights[end]);
            max = max.max((end as i32 - start as i32 + 1) * min_height);
        }
    }
    max
}
