fn main() {
    println!("{}", Solution::rob(vec![1, 2, 3, 1]));
    println!("{}", Solution::rob(vec![2, 7, 9, 3, 1]));
}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max_if_finishing_at_a_house = vec![0; nums.len()];
        for day in 0..nums.len() {
            let max_if_robbing_second_prev_house = day
                .checked_sub(2)
                .and_then(|second_prev| Some(max_if_finishing_at_a_house[second_prev]))
                .unwrap_or(0);
            let max_if_robbing_prev_house = day
                .checked_sub(1)
                .and_then(|prev| Some(max_if_finishing_at_a_house[prev]))
                .unwrap_or(0);
            max_if_finishing_at_a_house[day] =
                max_if_robbing_prev_house.max(max_if_robbing_second_prev_house + nums[day]);
        }
        *max_if_finishing_at_a_house.last().unwrap()

        //// for the sake of doing it, this is how we could have used .fold() here instead
        // *(0..nums.len())
        //     .fold(
        //         vec![0; nums.len()],
        //         |mut max_if_finishing_at_a_house, day| {
        //             let max_if_robbing_second_prev_house = day
        //                 .checked_sub(2)
        //                 .and_then(|second_prev| Some(max_if_finishing_at_a_house[second_prev]))
        //                 .unwrap_or(0);
        //             let max_if_robbing_prev_house = day
        //                 .checked_sub(1)
        //                 .and_then(|prev| Some(max_if_finishing_at_a_house[prev]))
        //                 .unwrap_or(0);
        //             max_if_finishing_at_a_house[day] =
        //                 max_if_robbing_prev_house.max(max_if_robbing_second_prev_house + nums[day]);
        //             max_if_finishing_at_a_house
        //         },
        //     )
        //     .last()
        //     .unwrap()
    }
}
