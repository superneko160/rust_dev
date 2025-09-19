use super::DeliverySubject;
use crate::observer::DeliveryObserver;
use crate::delivery_status::DeliveryStatus;

// ConcreteSubject

pub struct DeliveryTracker {
    observers: Vec<Box<dyn DeliveryObserver>>,
}

impl DeliveryTracker {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn set_delivery_status(&self, status: &dyn DeliveryStatus) {
        self.notify_observers(status);
    }
}

impl DeliverySubject for DeliveryTracker {
    fn add_observer(&mut self, observer: Box<dyn DeliveryObserver>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, status: &dyn DeliveryStatus) {
        for observer in &self.observers {
            observer.update(status);
        }
    }
}
