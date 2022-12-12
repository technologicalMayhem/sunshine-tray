use std::{thread, time::Duration};

use sunshine_tray::TrayIcon;

/// Create a new tray icon and updates the state of it every 2 seconds.
fn main() {
    let service = ksni::TrayService::new(TrayIcon::new());
    let handle = service.handle();
    service.spawn();

    loop {
        handle.update(|icon: &mut TrayIcon| icon.update());
        thread::sleep(Duration::from_secs(2));
    }
}
