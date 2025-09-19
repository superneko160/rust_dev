use super::DeliveryStatus;
use std::fmt;

#[derive(Debug)]
pub struct Transporting;

impl fmt::Display for Transporting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "配達中")
    }
}

impl DeliveryStatus for Transporting {}
