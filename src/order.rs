use chrono::{DateTime, Utc};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Order {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub price: f64,
    pub quantity: i32,
}

impl PartialEq<Self> for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.timestamp == other.timestamp
    }
}

impl Eq for Order {}

#[derive(Debug)]

pub struct BuyOrder {
    pub order: Order,
}

impl BuyOrder {
    pub fn new(id: String, timestamp: DateTime<Utc>, price: f64, quantity: i32) -> Self {
        Self {
            order: Order {
                id,
                timestamp,
                price,
                quantity,
            }
        }
    }
}

#[derive(Debug)]

pub struct SellOrder {
    pub order: Order,
}

impl SellOrder {
    pub fn new(id: String, timestamp: DateTime<Utc>, price: f64, quantity: i32) -> Self {
        Self {
            order: Order {
                id,
                timestamp,
                price,
                quantity,
            }
        }
    }
}

impl PartialEq<Self> for BuyOrder {
    fn eq(&self, other: &Self) -> bool {
        self.order.eq(&other.order)
    }
}

impl Eq for BuyOrder {}

impl PartialOrd<Self> for BuyOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = self.order.price.partial_cmp(&other.order.price);
        if let Some(res) = res {
            if res == Ordering::Equal {
                return other.order.timestamp.partial_cmp(&self.order.timestamp);
            }
        }

        res
    }
}

impl Ord for BuyOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq<Self> for SellOrder {
    fn eq(&self, other: &Self) -> bool {
        self.order.eq(&other.order)
    }
}

impl Eq for SellOrder {}

impl PartialOrd<Self> for SellOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = other.order.price.partial_cmp(&self.order.price);
        if let Some(res) = res {
            if res == Ordering::Equal {
                return other.order.timestamp.partial_cmp(&self.order.timestamp);
            }
        }

        res
    }
}

impl Ord for SellOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
