struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::MAX;
        let mut profit = 0;

        for price in prices {
            buy = buy.min(price);
            profit = profit.max(price - buy);
        }

        profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let max_profit = Solution::max_profit(prices);
    assert_eq!(max_profit, 5);

    let prices = vec![7, 6, 4, 3, 1];
    let max_profit = Solution::max_profit(prices);
    assert_eq!(max_profit, 0);
}