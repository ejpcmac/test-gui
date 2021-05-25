use std::time::Duration;

use ercp_basic::{
    adapter::SerialPortAdapter, command::ACK, error::CommandError, Command,
    DefaultRouter, ErcpBasic, Error,
};

/// A development device.
pub struct Device {
    ercp: ErcpBasic<SerialPortAdapter, DefaultRouter, 255>,
}

pub struct Color {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

const UPDATE_COLOR: u8 = 0x20;

impl Device {
    /// Creates a new device.
    pub fn new(port: &str) -> Result<Self, serialport::Error> {
        let port = serialport::new(port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()?;

        let device = Self {
            ercp: ErcpBasic::new(SerialPortAdapter::new(port), DefaultRouter),
        };

        Ok(device)
    }

    /// Pings the device.
    pub fn ping(&mut self) -> Result<(), Error> {
        self.ercp.ping()
    }

    /// Updates the hue.
    pub fn update_color(&mut self, color: &Color) -> Result<(), Error> {
        let value = [color.hue, color.sat, color.val];
        let command = Command::new(UPDATE_COLOR, &value)?;
        let reply = self.ercp.command(command)?;

        if reply.command() == ACK {
            Ok(())
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }
}
