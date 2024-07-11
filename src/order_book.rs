use crate::order::{BuyOrder, SellOrder};
use std::collections::BinaryHeap;

pub struct OrderBook {
    buy_orders: BinaryHeap<BuyOrder>,
    sell_orders: BinaryHeap<SellOrder>,
    log: Vec<String>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buy_orders: Default::default(),
            sell_orders: Default::default(),
            log: vec![],
        }
    }

    pub fn print_log(&self) {
        for e in &self.log {
            println!("{e}");
        }
    }

    fn log_match(&mut self, buy_ord: &BuyOrder, sell_ord: &SellOrder) {
        self.log.push(format!(
            "{} {} {} {}",
            buy_ord.order.id,
            sell_ord.order.price,
            buy_ord.order.quantity.min(sell_ord.order.quantity),
            sell_ord.order.id
        ));
    }

    pub fn add_buy_order(&mut self, buy_ord: BuyOrder) {
        if let Some(sell_ord) = self.sell_orders.peek() {
            if sell_ord.order.price > buy_ord.order.price {
                self.buy_orders.push(buy_ord);
                return;
            }
            let mut sell_ord = self.sell_orders.pop().unwrap();

            self.log_match(&buy_ord, &sell_ord);
            if sell_ord.order.quantity > buy_ord.order.quantity {
                sell_ord.order.quantity -= buy_ord.order.quantity;
                self.sell_orders.push(sell_ord);
            } else if sell_ord.order.quantity < buy_ord.order.quantity {
                let mut buy_ord = buy_ord;
                buy_ord.order.quantity -= sell_ord.order.quantity;
                self.add_buy_order(buy_ord);
            }

        } else {
            self.buy_orders.push(buy_ord);
        }
    }

    pub fn add_sell_order(&mut self, sell_ord: SellOrder) {
        if let Some(buy_ord) = self.buy_orders.peek() {
            if sell_ord.order.price > buy_ord.order.price {
                self.sell_orders.push(sell_ord);
                return;
            }

            let mut buy_ord = self.buy_orders.pop().unwrap();
            self.log_match(&buy_ord, &sell_ord);

            if sell_ord.order.quantity < buy_ord.order.quantity {
                buy_ord.order.quantity -= sell_ord.order.quantity;
                self.buy_orders.push(buy_ord);
            } else if sell_ord.order.quantity > buy_ord.order.quantity {
                let mut sell_ord = sell_ord;
                sell_ord.order.quantity -= buy_ord.order.quantity;
                self.add_sell_order(sell_ord);
            }

        } else {
            self.sell_orders.push(sell_ord);
        }
    }
}
