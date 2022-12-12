# Sunshine Tray Icon
## A tray icon to manage [Sunshine](https://github.com/LizardByte/Sunshine)
A simple tray icon that shows the current status of sunshine as well as provided the ability to quickly start, stop or configure Sunshine.

So far it only runs on linux and assumes you have a created a service file for systemd to run sunshine. If anyone wants support for the other installation options or other platforms, just let me know and I'll se if I can implement it.

## Usage

Use cargo to install the application
```
cargo install sunshine-tray-icon
```

Then simply start it up by running ``sunshine-tray-icon``.

I would recommend either creating a start menu icon or put it into your system autostart.

## Attribution
### Icons
Sun icon created by [Good Ware - Flaticon](https://www.flaticon.com/free-icons/sun). Variations made by me.

### Crates
 - [ksni](https://crates.io/crates/ksni) by iovxw
 - [notify-rust](https://crates.io/crates/notify-rust) by hoodie
 - [image](https://crates.io/crates/image) by image-rs
 - [const-gen](https://crates.io/crates/const-gen) by Eolu