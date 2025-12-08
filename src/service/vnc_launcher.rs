use crate::model::VncConnection;
use std::process::{Child, Command};
use std::{env, io};

pub struct VncLauncher;

impl VncLauncher {
    pub fn launch(connection: &mut VncConnection) -> io::Result<Child> {
        let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
        let target = connection.address();
        if session_type.to_lowercase().contains("wayland") {
            Command::new("wayvnc").arg("-c").arg(&target).spawn()
        } else {
            Command::new("x11vnc")
                .arg("-connect")
                .arg(&target)
                .arg("-display")
                .arg(":0")
                .arg("-ncache")
                .arg("10")
                .arg("-once")
                .arg("-nopw")
                .spawn()
        }
    }
}
