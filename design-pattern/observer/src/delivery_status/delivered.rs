use super::DeliveryStatus;
use std::fmt;

#[derive(Debug)]
pub struct Delivered;

impl fmt::Display for Delivered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "配達完了（配達済み）")
    }
}

impl DeliveryStatus for Delivered {}
