use super::DeliveryStatus;
use std::fmt;

#[derive(Debug)]
pub struct ReadyForShipment;

impl fmt::Display for ReadyForShipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "配達可能（配送準備完了）")
    }
}

impl DeliveryStatus for ReadyForShipment {}
