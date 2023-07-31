#![allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = std::i32::MAX;
    let mut max_profit = 0;

    for price in prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }

    max_profit
}

/*
    Algorithm - Kadane's Algorithm

    Time Complexity = O(n)
    Space Complexity = O(1)

    Explanation:
    1. Iterate through the prices vector
    2. If the current price is less than the minimum price, set the minimum price to the current price
    3. Else if the current price - minimum price is greater than the maximum profit, set the maximum profit to the current price - minimum price
    4. Return the maximum profit
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
