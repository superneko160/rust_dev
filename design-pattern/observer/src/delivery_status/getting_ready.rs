use super::DeliveryStatus;
use std::fmt;

#[derive(Debug)]
pub struct GettingReady;

impl fmt::Display for GettingReady {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "準備中")
    }
}

impl DeliveryStatus for GettingReady {}
