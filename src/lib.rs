use std::{
    process::{Command, Output, self},
    vec,
};

use ksni::{menu::StandardItem, Icon, MenuItem, Tray};
use notify_rust::Notification;

//Imports generated constants for the images
include!(concat!(env!("OUT_DIR"), "/const_images.rs"));

pub struct TrayIcon {
    icon: Icon,
    state: SunshineState,
    report_shutdown: ReportShutdown,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SunshineState {
    Off,
    Ready,
    Active,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ReportShutdown {
    Report,
    DoNotReport,
}

impl TrayIcon {
    pub fn new() -> Self {
        let state = poll_status();
        TrayIcon {
            icon: create_icon(state),
            state,
            report_shutdown: ReportShutdown::DoNotReport,
        }
    }

    fn update_state(&mut self, new_state: SunshineState) {
        if self.state == new_state {
            return;
        }

        match new_state {
            SunshineState::Off => {
                if self.report_shutdown == ReportShutdown::Report {
                    display_notification(
                        "Sunshine stopped",
                        "Sunshine stopped running unexpectedly.",
                    );
                }
            }
            SunshineState::Ready => {
                if self.report_shutdown != ReportShutdown::Report {
                    self.report_shutdown = ReportShutdown::Report
                }

                if self.state == SunshineState::Off {
                    display_notification(
                        "Client disconnected",
                        "A client disconnected from Sunshine.",
                    );
                }

                if self.state == SunshineState::Active {
                    display_notification(
                        "Client disconnected",
                        "A client disconnected from Sunshine.",
                    );
                }
            }
            SunshineState::Active => {
                display_notification("Client connected", "A client connected to Sunshine.");
            }
        }

        self.state = new_state;
        self.icon = create_icon(self.state);
    }

    pub fn update(&mut self) {
        self.update_state(poll_status())
    }
}

fn create_icon(state: SunshineState) -> Icon {
    Icon {
        height: 64,
        width: 64,
        data: Vec::from(match state {
            SunshineState::Off => IMAGE_OFF,
            SunshineState::Ready => IMAGE_READY,
            SunshineState::Active => IMAGE_ACTIVE,
        }),
    }
}

impl Tray for TrayIcon {
    fn icon_pixmap(&self) -> Vec<Icon> {
        vec![self.icon.clone()]
    }

    fn title(&self) -> String {
        "Sunshine".into()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        if self.state != SunshineState::Off {
            open_configuration()
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let mut v: Vec<MenuItem<Self>> = Vec::new();

        v.push(
            StandardItem {
                label: "Configuration".into(),
                enabled: self.state != SunshineState::Off,
                activate: Box::new(|_| open_configuration()),
                icon_name: "configure".into(),
                ..Default::default()
            }
            .into(),
        );

        v.push(MenuItem::Separator);

        v.push(
            StandardItem {
                label: "Restart".into(),
                enabled: self.state != SunshineState::Off,
                activate: Box::new(|_| restart_sunshine()),
                icon_name: "view-refresh".into(),
                ..Default::default()
            }
            .into(),
        );
        if self.state == SunshineState::Off {
            v.push(
                StandardItem {
                    label: "Startup".into(),
                    activate: Box::new(|_| start_sunshine()),
                    icon_name: "media-playback-start".into(),
                    ..Default::default()
                }
                .into(),
            );
        }
        if self.state != SunshineState::Off {
            v.push(
                StandardItem {
                    label: "Shutdown".into(),
                    activate: Box::new(|_| stop_sunshine()),
                    icon_name: "kt-stop".into(),
                    ..Default::default()
                }
                .into(),
            );
        }

        v.push(MenuItem::Separator);

        v.push(
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| process::exit(0)),
                icon_name: "gtk-quit".into(),
                ..Default::default()
            }
            .into(),
        );

        v
    }
}

fn open_configuration() {
    let mut p = Command::new("xdg-open")
        .arg("https://localhost:47990/")
        .spawn()
        .expect("Failed to spawn xdg-open.");
    p.wait().expect("Process did not exit succefully.");
}

fn poll_status() -> SunshineState {
    if check_systemctl() == false {
        return SunshineState::Off;
    }

    if check_for_stream_thread() {
        return SunshineState::Active;
    } else {
        return SunshineState::Ready;
    }
}

fn check_systemctl() -> bool {
    let out = create_process("systemctl", vec!["--user", "status", "sunshine"]);

    out.status.success()
}

fn check_for_stream_thread() -> bool {
    let out = create_process("ps", vec!["-T", "-C", "sunshine"]);
    let out_s = String::from_utf8_lossy(&out.stdout);

    out_s.contains("threaded-ml")
}

fn start_sunshine() {
    systemctl_action("start");
}

fn stop_sunshine() {
    systemctl_action("stop");
}

fn restart_sunshine() {
    systemctl_action("restart");
}

fn systemctl_action(action: &str) -> Output {
    create_process("systemctl", vec!["--user", action, "sunshine"])
}

fn create_process(file: &str, args: Vec<&str>) -> Output {
    Command::new(file)
        .args(args)
        .output()
        .expect(&format!("Failed to execute {}.", file))
}

fn display_notification(summary: &str, body: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .appname("Sunshine Status")
        .show()
        .expect("Unable to show notfication.");
}
