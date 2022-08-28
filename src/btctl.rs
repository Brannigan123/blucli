use std::fmt;
use std::io::Error;
use std::process::{Command, Output};


/// `Device` is a struct that contains a `String` called `alias` and a `String` called `mac_address`.
/// 
/// Properties:
/// 
/// * `alias`: The name of the device.
/// * `mac_address`: The MAC address of the device.
#[derive(Debug)]
#[derive(Clone)]
pub struct Device {
    pub alias: String,
    pub mac_address: String,
}

/// It's a trait implementation. It's defining how to print a Device struct.
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.alias, self.mac_address)
    }
}

/// It takes a vector of strings, and returns a Result<Output, Error> from the command "bluetoothctl"
/// with the arguments in the vector
/// 
/// Arguments:
/// 
/// * `arg`: The arguments to pass to bluetoothctl.
/// 
/// Returns:
/// 
/// A Result<Output, Error>
pub fn exec_btctl(arg: Vec<&str>) -> Result<Output, Error> {
    Command::new("bluetoothctl").args(arg).output()
}

/// It runs `bluetoothctl devices` and parses the output into a vector of `Device` structs
/// 
/// Returns:
/// 
/// A vector of devices.
pub fn devices() -> Result<Vec<Device>, Error> {
    exec_btctl(vec!["devices"]).map(|output| {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .map(|splits| Device {
                alias: splits[2].to_string(),
                mac_address: splits[1].to_string(),
            })
            .collect::<Vec<Device>>()
    })
}
