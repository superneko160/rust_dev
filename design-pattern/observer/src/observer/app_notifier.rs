use super::DeliveryObserver;
use crate::delivery_status::DeliveryStatus;
use crate::utils::get_current_datetime;

// ConcreteObserver

pub struct AppNotifier;

impl DeliveryObserver for AppNotifier {
    fn update(&self, status: &dyn DeliveryStatus) {
        let datetime = get_current_datetime();
        println!("{} [アプリ通知]配送ステータスが更新されました: {}", datetime, status);
    }
}
