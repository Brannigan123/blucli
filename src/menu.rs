use crate::btctl::{available_devices, devices, exec_btctl, Device};
use colored::Colorize;
use inquire::formatter::{MultiOptionFormatter, OptionFormatter};
use inquire::{list_option::ListOption, validator::Validation};
use inquire::{MultiSelect, Select};
use std::fmt;

/// Creating an enum called `Stage` with three variants: `DeviceSelection`, `ActionSelection`, and
/// `Exit`.
#[derive(Debug)]
pub enum Stage {
    StageSelection,
    DeviceSelection,
    AvailableDeviceSelection,
    ActionSelection,
    Exit,
}

/// This is an implementation of the `Display` trait for the `Stage` enum.
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::StageSelection => write!(f, "Choose what to do next"),
            Stage::DeviceSelection => write!(f, "Select device(s)"),
            Stage::AvailableDeviceSelection => write!(f, "Find available device(s)"),
            Stage::ActionSelection => write!(f, "Perform actions on selected device(s)"),
            Stage::Exit => write!(f, "Exit"),
        }
    }
}

/// Implementing the `Display` trait for the `Stage` enum.
/// `device_selection` returns a `Vec<Device>` by using `MultiSelect` to prompt the user to select one
/// or more devices from a list of devices returned by `devices()`
///
/// Returns:
///
/// A vector of devices
fn device_selection(get_available: bool) -> Vec<Device> {
    let device_options = if get_available {
        available_devices().expect("Failed to get available device using hcitool!")
    } else {
        devices().expect("Failed to get device list using bluetoothctl!")
    };
    if device_options.is_empty() {
        device_options
    } else {
        let formatter: MultiOptionFormatter<Device> =
            &|opts| format!("Selected {} device(s)", opts.len());
        let validator = |opts: &[ListOption<&Device>]| {
            Ok(if opts.len() <= 0 {
                Validation::Invalid("Select at least 1 device!".into())
            } else {
                Validation::Valid
            })
        };
        MultiSelect::new("Select device(s): ", device_options)
            .with_validator(validator)
            .with_formatter(formatter)
            .prompt()
            .expect("Failed to capture selection(s)")
    }
}

/// `actions_selection` is a function that takes a `device_count` as an argument and returns a `String`
/// representing action selected by the user.
///
/// Arguments:
///
/// * `device_count`: The number of devices that were found.
///
/// Returns:
///
/// A string representing an action
fn actions_selection(device_count: usize) -> String {
    let formatter: OptionFormatter<&str> = &|a| format!("Chose to {a} {device_count} device(s)");
    let actions = vec![
        "connect",
        "disconnect",
        "pair",
        "trust",
        "untrust",
        "block",
        "unblock",
        "remove",
    ];
    Select::new("What do you want to do?", actions)
        .with_formatter(formatter)
        .prompt()
        .expect("Failed to capture selection(s)")
        .to_string()
}

/// Prompts user for the next stage to execute
///
/// Returns:
///
/// user selected Stage to executed next
fn select_next_stage(device_count: usize) -> Stage {
    let formatter: OptionFormatter<Stage> = &|a| format!("Chose to {a}");
    let actions = if device_count > 0 {
        vec![
            Stage::DeviceSelection,
            Stage::AvailableDeviceSelection,
            Stage::ActionSelection,
            Stage::Exit,
        ]
    } else {
        vec![
            Stage::DeviceSelection,
            Stage::AvailableDeviceSelection,
            Stage::Exit,
        ]
    };
    Select::new("What would you like to do next?", actions)
        .with_formatter(formatter)
        .prompt()
        .expect("Failed to capture selection(s)")
}

/// It runs a loop that switches between two stages: device selection and action selection
pub fn run() {
    let mut stage = Stage::StageSelection;
    let mut selections = Vec::new();
    exec_btctl(vec!["power", "on"]).expect("Failed to power on via bluetoothctl");
    loop {
        match stage {
            Stage::DeviceSelection => selections = device_selection(false),
            Stage::AvailableDeviceSelection => selections = device_selection(true),
            Stage::ActionSelection => {
                let action = actions_selection(selections.len());
                for device in &selections {
                    let name = &device.alias;
                    let mac_address = &device.mac_address;
                    let res = exec_btctl(vec![&action, &mac_address]);
                    match res {
                        Ok(output) => println!(
                            "\n{}",
                            format!("{}", String::from_utf8_lossy(&output.stdout))
                                .purple()
                                .bold()
                        ),
                        Err(_) => {
                            println!("{}", format!("Failed to {} {}", action, name).red().bold())
                        }
                    }
                }
            }
            Stage::StageSelection => {
                stage = select_next_stage(selections.len());
                continue;
            }
            Stage::Exit => break,
        }
        stage = Stage::StageSelection
    }
}
