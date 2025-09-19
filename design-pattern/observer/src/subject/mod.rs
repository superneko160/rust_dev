use crate::observer::DeliveryObserver;
use crate::delivery_status::DeliveryStatus;

// Subject
pub trait DeliverySubject {
    fn add_observer(&mut self, observer: Box<dyn DeliveryObserver>);
    fn notify_observers(&self, status: &dyn DeliveryStatus);
}

pub mod delivery_tracker;

pub use delivery_tracker::DeliveryTracker;
