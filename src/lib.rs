use std::time::Duration;

use rusb::{request_type, Direction, GlobalContext, Recipient, RequestType, UsbContext};

const PIXELRING_USB_VID: u16 = 0x2886;
const PIXELRING_USB_PID: u16 = 0x0018;

#[repr(u8)]
#[allow(dead_code)]
enum Command {
    Trace = 0x00,
    Mono = 0x01,
    Listen = 0x02,
    Speak = 0x03,
    Think = 0x04,
    Spin = 0x05,
    Show = 0x06,
    Brightness = 0x20,
    ColorPalette = 0x21,
    VadLed = 0x22,
    Volume = 0x23,
}

impl From<Command> for u16 {
    fn from(command: Command) -> Self {
        command as u16
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// USB error.
    Usb(rusb::Error),
}

impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Error::Usb(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

pub struct PixelRing {
    handle: rusb::DeviceHandle<rusb::GlobalContext>,
}

impl PixelRing {
    pub fn new() -> Option<Self> {
        let ctx: GlobalContext = GlobalContext {};
        let devices = match ctx.devices() {
            Ok(d) => d,
            Err(_) => return None,
        };

        for device in devices.iter() {
            let desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue,
            };

            if desc.vendor_id() == PIXELRING_USB_VID && desc.product_id() == PIXELRING_USB_PID {
                match device.open() {
                    Ok(handle) => return Some(PixelRing { handle }),
                    Err(_) => continue,
                }
            }
        }
        None
    }

    pub fn trace(&self) -> Result<(), Error> {
        self.write_command(Command::Trace, &[0])
    }

    pub fn mono(&self, r: u8, g: u8, b: u8) -> Result<(), Error> {
        self.write_command(Command::Mono, &[r, g, b, 0])
    }

    pub fn listen(&self) -> Result<(), Error> {
        self.write_command(Command::Listen, &[0])
    }

    pub fn speak(&self) -> Result<(), Error> {
        self.write_command(Command::Speak, &[0])
    }

    pub fn think(&self) -> Result<(), Error> {
        self.write_command(Command::Think, &[0])
    }

    pub fn spin(&self) -> Result<(), Error> {
        self.write_command(Command::Spin, &[0])
    }

    pub fn show(&self, data: &[u8]) -> Result<(), Error> {
        self.write_command(Command::Show, data)
    }

    pub fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        self.write_command(Command::Brightness, &[brightness])
    }

    fn write_command(&self, cmd: Command, buf: &[u8]) -> Result<(), Error> {
        let request_type = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let request = 0;
        let value = cmd.into();
        let index = 0x1c;
        let timeout = Duration::from_secs(1);

        self.handle
            .write_control(request_type, request, value, index, buf, timeout)?;

        Ok(())
    }
}
