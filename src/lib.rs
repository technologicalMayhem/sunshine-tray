use std::{
    process::{Command, Output, self},
    vec,
};

use ksni::{menu::StandardItem, Icon, MenuItem, Tray};
use notify_rust::Notification;

//Imports generated constants for the images
include!(concat!(env!("OUT_DIR"), "/const_images.rs"));

/// This keeps track of the tray icons current state.
pub struct TrayIcon {
    /// The icon that should be displayed right now.
    icon: Icon,
    /// The state of the Sunshine daemon.
    daemon_state: SunshineState,
    /// Whether or not to report a shutdown to the user.
    report_shutdown: ReportShutdown,
}

/// The current state of the Sunshine daemon.
#[derive(Clone, Copy, PartialEq, Debug)]
enum SunshineState {
    /// The daemon is stopped.
    Stopped,
    /// The daeomon is running.
    Running,
    /// The daemon has a client connected to it.
    ClientConnected,
}

/// Whether or not to report a shutdown to the user via a notification.
#[derive(Clone, Copy, PartialEq, Debug)]
enum ReportShutdown {
    /// Report a shutdown.
    Report,
    /// Do not report a shutdown.
    DoNotReport,
}

impl TrayIcon {
    /// Creates a new [`TrayIcon`].
    pub fn new() -> Self {
        let daemon_state = poll_status();
        TrayIcon {
            icon: create_icon(daemon_state),
            daemon_state,
            report_shutdown: ReportShutdown::DoNotReport,
        }
    }

    /// Update the tray icon to refect the daemons current state.
    fn update_state(&mut self, new_state: SunshineState) {
        if self.daemon_state == new_state {
            return;
        }

        self.create_notifications(new_state);

        self.daemon_state = new_state;
        self.icon = create_icon(self.daemon_state);
    }

    /// Create notifications, if necesarry.
    fn create_notifications(&mut self, new_state: SunshineState) {
        match new_state {
            SunshineState::Stopped => {
                if self.report_shutdown == ReportShutdown::Report {
                    display_notification(
                        "Sunshine stopped",
                        "Sunshine stopped running unexpectedly.",
                    );
                }
            }
            SunshineState::Running => {
                if self.report_shutdown != ReportShutdown::Report {
                    self.report_shutdown = ReportShutdown::Report
                }

                if self.daemon_state == SunshineState::Stopped {
                    display_notification(
                        "Client disconnected",
                        "A client disconnected from Sunshine.",
                    );
                }

                if self.daemon_state == SunshineState::ClientConnected {
                    display_notification(
                        "Client disconnected",
                        "A client disconnected from Sunshine.",
                    );
                }
            }
            SunshineState::ClientConnected => {
                display_notification("Client connected", "A client connected to Sunshine.");
            }
        }
    }

    pub fn update(&mut self) {
        self.update_state(poll_status())
    }
}

/// Create icon for given state of sunshine.
fn create_icon(state: SunshineState) -> Icon {
    Icon {
        height: 64,
        width: 64,
        data: Vec::from(match state {
            SunshineState::Stopped => IMAGE_OFF,
            SunshineState::Running => IMAGE_READY,
            SunshineState::ClientConnected => IMAGE_ACTIVE,
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
        if self.daemon_state != SunshineState::Stopped {
            open_configuration()
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let mut v: Vec<MenuItem<Self>> = Vec::new();

        //Add button to open config menu
        v.push(
            StandardItem {
                label: "Configuration".into(),
                enabled: self.daemon_state != SunshineState::Stopped,
                activate: Box::new(|_| open_configuration()),
                icon_name: "configure".into(),
                ..Default::default()
            }
            .into(),
        );

        v.push(MenuItem::Separator);

        //Add restart button
        v.push(
            StandardItem {
                label: "Restart".into(),
                enabled: self.daemon_state != SunshineState::Stopped,
                activate: Box::new(|_| restart_sunshine()),
                icon_name: "view-refresh".into(),
                ..Default::default()
            }
            .into(),
        );
        //Add startup if stopped
        if self.daemon_state == SunshineState::Stopped {
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
        //Add shutdown if running
        if self.daemon_state != SunshineState::Stopped {
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

        //Add button to close the tray icon
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

/// Opens the configuration interface for Sunshine in the users web browser.
fn open_configuration() {
    create_process("xdg-open", vec!["https://localhost:47990/"]);
}

/// Poll the current state of the Sunshine daemon.
fn poll_status() -> SunshineState {
    if check_systemctl() == false {
        return SunshineState::Stopped;
    }

    if check_for_stream_thread() {
        return SunshineState::ClientConnected;
    } else {
        return SunshineState::Running;
    }
}

/// Check if the Sunshine daemon is running using systemctl.
fn check_systemctl() -> bool {
    let out = create_process("systemctl", vec!["--user", "status", "sunshine"]);

    out.status.success()
}

/// Checks if a client is currently connected to the Sunshine daemon.
/// 
/// This is accomplished by checking if sunshine has a thread running with the name 'threaded-ml'.
fn check_for_stream_thread() -> bool {
    let out = create_process("ps", vec!["-T", "-C", "sunshine"]);
    let out_s = String::from_utf8_lossy(&out.stdout);

    out_s.contains("threaded-ml")
}

/// Starts the Sunshine daemon.
fn start_sunshine() {
    systemctl_action("start");
}

/// Stops the Sunshine daemon.
fn stop_sunshine() {
    systemctl_action("stop");
}

/// Restarts the Sunshine daemon.
fn restart_sunshine() {
    systemctl_action("restart");
}

/// Peforms the given action on the Sunshine daemon.
fn systemctl_action(action: &str) -> Output {
    create_process("systemctl", vec!["--user", action, "sunshine"])
}

/// Helper method to quickly create a program with the given arguments.
///
/// # Panics
///
/// Panics if it fails to execute the program.
fn create_process(file: &str, args: Vec<&str>) -> Output {
    Command::new(file)
        .args(args)
        .output()
        .expect(&format!("Failed to execute {}.", file))
}

/// Displays a notfication on the desktop.
///
/// # Panics
///
/// Panics if it is unable to create a notification.
fn display_notification(summary: &str, body: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .appname("Sunshine Status")
        .show()
        .expect("Unable to show notfication.");
}
