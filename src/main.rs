use crate::order::{BuyOrder, SellOrder};
use chrono::{TimeDelta, Utc};
use std::ops::Add;
use crate::order_book::OrderBook;

mod order;
mod order_book;

fn main() {
    let mut ob = OrderBook::new();
    // let o1 = BuyOrder::new(String::from("1"), Utc::now(), 237.45, 100);
    // let o2 = BuyOrder::new(String::from("2"), Utc::now().add(TimeDelta::minutes(1)), 237.45, 90);
    // println!("{}", o1 < o2);

    ob.add_sell_order(SellOrder::new(String::from("1"), Utc::now(), 240.12, 100));
    ob.add_sell_order(SellOrder::new(String::from("2"), Utc::now().add(TimeDelta::minutes(1)), 237.45, 90));
    ob.add_buy_order(BuyOrder::new(String::from("3"), Utc::now().add(TimeDelta::minutes(2)), 238.10, 110));
    ob.add_buy_order(BuyOrder::new(String::from("4"), Utc::now().add(TimeDelta::minutes(3)), 237.80, 10));
    ob.add_buy_order(BuyOrder::new(String::from("5"), Utc::now().add(TimeDelta::minutes(4)), 237.80, 40));
    ob.add_sell_order(SellOrder::new(String::from("6"), Utc::now().add(TimeDelta::minutes(5)), 236.00, 50));

    ob.print_log();

}
