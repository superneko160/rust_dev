use std::fmt;

pub trait DeliveryStatus: fmt::Display + fmt::Debug  {}

pub mod getting_ready;
pub mod ready_for_shipment;
pub mod transporting;
pub mod delivered;

pub use getting_ready::GettingReady;
pub use ready_for_shipment::ReadyForShipment;
pub use transporting::Transporting;
pub use delivered::Delivered;
