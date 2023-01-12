fn main() {
    // println!("{}", Solution::max_profit(vec![3, 4, 20, 21, 1, 9]));
    // println!("{}", Solution::max_profit(vec![20, 21, 3, 4, 1, 9]));
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    // println!("{}", Solution::max_profit(vec![1, 2, 3, 4, 5]));
    // println!(
    //     "{}",
    //     Solution::max_profit(vec![
    //         1, 2, 3, 4, //
    //         1, 2, 3, 4, 5
    //     ])
    // );
    // println!("{}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .enumerate()
            .rev()
            .fold(
                vec![],
                |mut best_profits_if_buying_on_or_after_following_days, (index, price)| {
                    // I need to compose a list of best profit for a day if I buy on or after that day
                    // and then, for each preceding day, I can say HEY
                    // the best profit if I buy on or after today is the max of the following:
                    // 1. the best profit if do nothing today and buy on or after any following day (the max of the existing items)
                    // 2. the profit if I buy today and sell on any following day i, plus the best profit on/after day i+1

                    // println!("price: {}", price);

                    let best_profit_if_not_buying_today =
                        *(best_profits_if_buying_on_or_after_following_days
                            .iter()
                            .max()
                            .unwrap_or(&0));
                    // println!(
                    //     "best_profit_if_not_buying_today: {}",
                    //     best_profit_if_not_buying_today
                    // );

                    let best_profit_if_buying_today = (1
                        ..=best_profits_if_buying_on_or_after_following_days.len())
                        .take_while(|i| prices[index + *i] > *price)
                        .map(|i| {
                            let best_profit_if_buying_today_and_selling_later = prices[index + i]
                                - price
                                + *(best_profits_if_buying_on_or_after_following_days
                                    .iter()
                                    .skip(i)
                                    .max()
                                    .unwrap_or(&0));
                            // println!(
                            //     "  best_profit_if_buying_today_and_selling_later on day {}: {}",
                            //     index + i,
                            //     best_profit_if_buying_today_and_selling_later
                            // );
                            best_profit_if_buying_today_and_selling_later
                        })
                        .max()
                        .unwrap_or(0);

                    // println!(
                    //     "best_profit_if_buying_today: {}",
                    //     best_profit_if_buying_today
                    // );

                    let best_profit_for_day =
                        best_profit_if_not_buying_today.max(best_profit_if_buying_today);
                    // println!("best_profit_for_day: {}", best_profit_for_day);

                    best_profits_if_buying_on_or_after_following_days
                        .insert(0, best_profit_for_day);
                    best_profits_if_buying_on_or_after_following_days
                },
            )
            .into_iter()
            .max()
            .unwrap()
    }
}
