fn main() {
    println!("{}", Solution::rob(vec![2, 3, 2]));
    println!("{}", Solution::rob(vec![1, 2, 3, 1]));
    println!("{}", Solution::rob(vec![1, 2, 3]));
    println!("{}", Solution::rob(vec![1]));
}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // we'll start at house 0 since that's intuitive
        // we can make this easier to fit in our heads if we consider two branching timelines to consider: one where we don't rob house 0, and one where we do. We can then just take the best outcome from both timelines

        let mut max_if_finishing_at_a_house_after_not_robbing_0 = vec![0; nums.len()];
        // let's the possibilities extending from not robbing house 0, we can start iterating with house 1
        // although the code nominally considers stealing from house 0 in the first two iterations, we've already set in stone that the max we can rob from house 0 is, well, 0, so naturally max_if_finishing_at_a_house_after_not_robbing_0[1] will organically become nums[1] and then things can flow normally from there
        // and since wee've set in stone that we're not robbing house 0, we don't have to worry about adjacency when considering whether to rob the last house
        for house in 1..nums.len() {
            let max_if_robbing_second_prev_house = house
                .checked_sub(2)
                .and_then(|second_prev| {
                    Some(max_if_finishing_at_a_house_after_not_robbing_0[second_prev])
                })
                .unwrap_or(0);
            let max_if_robbing_prev_house = house
                .checked_sub(1)
                .and_then(|prev| Some(max_if_finishing_at_a_house_after_not_robbing_0[prev]))
                .unwrap_or(0);
            max_if_finishing_at_a_house_after_not_robbing_0[house] =
                max_if_robbing_prev_house.max(max_if_robbing_second_prev_house + nums[house]);
        }
        // println!(
        //     "max_if_finishing_at_a_house_after_not_robbing_0: {:?}",
        //     max_if_finishing_at_a_house_after_not_robbing_0
        // );

        // now let's explore the possibilities extending from robbing house 0. Once more, we'll start iterating at house 1
        // since we're robbing house 0, we can't rob the last house, so we'll only iterate to the penultimate house (if there are only one or two houses, that's fine)
        let mut max_if_finishing_at_a_house_after_robbing_0 = vec![0; nums.len()];
        // before we start iterating, we need to set in stone that we are in fact robbing house 0
        max_if_finishing_at_a_house_after_robbing_0[0] = nums[0];
        for house in 1..nums.len() - 1 {
            let max_if_robbing_second_prev_house = house
                .checked_sub(2)
                .and_then(|second_prev| {
                    Some(max_if_finishing_at_a_house_after_robbing_0[second_prev])
                })
                .unwrap_or(0);
            let max_if_robbing_prev_house = house
                .checked_sub(1)
                .and_then(|prev| Some(max_if_finishing_at_a_house_after_robbing_0[prev]))
                .unwrap_or(0);
            max_if_finishing_at_a_house_after_robbing_0[house] =
                max_if_robbing_prev_house.max(max_if_robbing_second_prev_house + nums[house]);
        }
        // since we robbed house 0, we can't consider robbing the last house... if there's more than one house at all, that is. If there is more than one house, let's just pretend it doesn't exist by popping it from the array. Alternatively, we could set it to have the same value as the penultimate house, but whatever
        if nums.len() > 1 {
            max_if_finishing_at_a_house_after_robbing_0.pop();
        }

        // println!(
        //     "max_if_finishing_at_a_house_after_robbing_0: {:?}",
        //     max_if_finishing_at_a_house_after_robbing_0
        // );

        *max_if_finishing_at_a_house_after_robbing_0
            .last()
            .max(max_if_finishing_at_a_house_after_not_robbing_0.last())
            .unwrap()
    }
}
