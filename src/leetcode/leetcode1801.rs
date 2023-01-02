use std::{cmp::Reverse, collections::BinaryHeap};

pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
    let mut buy: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut sell: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    for order in orders {
        let price = order[0];
        let mut amount = order[1];
        let order_type = order[2];
        if order_type == 0 {
            while amount > 0 && !sell.is_empty() && sell.peek().unwrap().0 .0 <= price {
                let Reverse((sell_price, sell_amount)) = sell.pop().unwrap();
                if sell_amount > amount {
                    sell.push(Reverse((sell_price, sell_amount - amount)));
                    amount = 0;
                } else {
                    amount -= sell_amount;
                }
            }
            if amount > 0 {
                buy.push((price, amount));
            }
        } else {
            while amount > 0 && !buy.is_empty() && buy.peek().unwrap().0 >= price {
                let (buy_price, buy_amount) = buy.pop().unwrap();
                if buy_amount > amount {
                    buy.push((buy_price, buy_amount - amount));
                    amount = 0;
                } else {
                    amount -= buy_amount;
                }
            }
            if amount > 0 {
                sell.push(Reverse((price, amount)));
            }
        }
    }
    let mut ans = 0;
    while !buy.is_empty() {
        ans += buy.pop().unwrap().1;
        ans %= 1000000007;
    }
    while !sell.is_empty() {
        ans += sell.pop().unwrap().0 .1;
        ans %= 1000000007;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_backlog_orders() {
        assert_eq!(
            get_number_of_backlog_orders(vec![
                vec![10, 5, 0],
                vec![15, 2, 1],
                vec![25, 1, 1],
                vec![30, 4, 0]
            ]),
            6
        );
        assert_eq!(
            get_number_of_backlog_orders(vec![
                vec![7, 1000000000, 1],
                vec![15, 3, 0],
                vec![5, 999999995, 0],
                vec![5, 1, 1]
            ]),
            999999984
        );

        assert_eq!(
            get_number_of_backlog_orders(vec![
                vec![1, 29, 1],
                vec![22, 7, 1],
                vec![24, 1, 0],
                vec![25, 15, 1],
                vec![18, 8, 1],
                vec![8, 22, 0],
                vec![25, 15, 1],
                vec![30, 1, 1],
                vec![27, 30, 0]
            ]),
            22
        );
    }
}
