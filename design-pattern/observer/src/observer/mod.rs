use crate::delivery_status::DeliveryStatus;

// Observer
pub trait DeliveryObserver {
    fn update(&self, status: &dyn DeliveryStatus);
}

pub mod app_notifier;
pub mod email_notifier;

pub use app_notifier::AppNotifier;
pub use email_notifier::EmailNotifier;
