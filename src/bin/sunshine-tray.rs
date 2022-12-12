use std::{env, thread, time::Duration};

use sunshine_tray::TrayIcon;

fn main() {
    env::set_var("SYSTEMCTL_PATH", "/usr/bin/systemctl");

    let service = ksni::TrayService::new(TrayIcon::new());
    let handle = service.handle();
    service.spawn();

    loop {
        handle.update(|icon: &mut TrayIcon| icon.update());
        thread::sleep(Duration::from_secs(2));
    }
}
