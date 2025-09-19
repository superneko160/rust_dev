use observer::delivery_status::*;
use observer::observer::*;
use observer::subject::*;
use std::time::Duration;

// Client
fn main() {
    // Concrete Subjectの生成
    let mut delivery_tracker = DeliveryTracker::new();

    // Concrete Observerを追加
    delivery_tracker.add_observer(Box::new(AppNotifier));
    delivery_tracker.add_observer(Box::new(EmailNotifier));

    // 配送ステータスを「準備中」に更新
    delivery_tracker.set_delivery_status(&GettingReady);
    println!("-------");
    std::thread::sleep(Duration::from_secs(2));

    // 配送ステータスを「準備中」に更新
    delivery_tracker.set_delivery_status(&ReadyForShipment);
    println!("-------");
    std::thread::sleep(Duration::from_secs(2));

    // 配送ステータスを「配達中」に更新
    delivery_tracker.set_delivery_status(&Transporting);
    println!("-------");
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 配送ステータスを「配達完了」に更新
    delivery_tracker.set_delivery_status(&Delivered);
}
