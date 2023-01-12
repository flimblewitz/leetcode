fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // naive(prices)
        v2(prices)
    }
}

// we can optimize this by skipping ahead whenever we hit a lower price to buy than what we're currently considering
fn v2(prices: Vec<i32>) -> i32 {
    let mut best_profit = 0;
    let mut i = 0;
    let mut j = 1;
    while i < prices.len() - 1 {
        while j < prices.len() {
            if prices[j] <= prices[i] {
                i = j;
            } else {
                best_profit = best_profit.max(prices[j] - prices[i]);
            }
            j += 1;
        }
        i += 1;
    }
    best_profit
}

fn naive(prices: Vec<i32>) -> i32 {
    let mut best_profit = 0;
    for i in 0..(prices.len() - 1) {
        for j in (i + 1)..prices.len() {
            best_profit = best_profit.max(prices[j] - prices[i]);
        }
    }
    best_profit
}
