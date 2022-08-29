use crate::btctl::{devices, exec_btctl, Device};
use colored::Colorize;
use inquire::formatter::{MultiOptionFormatter, OptionFormatter};
use inquire::{list_option::ListOption, validator::Validation};
use inquire::{MultiSelect, Select};
use std::fmt;

/// Creating an enum called `Stage` with three variants: `DeviceSelection`, `ActionSelection`, and
/// `Exit`.
#[derive(Debug)]
pub enum Stage {
    DeviceSelection,
    ActionSelection,
    Exit,
}

/// This is an implementation of the `Display` trait for the `Stage` enum.
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::DeviceSelection => write!(f, "go to device selection"),
            Stage::ActionSelection => write!(f, "perform actions on selected devices"),
            Stage::Exit => write!(f, "exit"),
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
fn device_selection() -> Vec<Device> {
    let device_options = devices().expect("Failed to get device list using bluetoothctl!");
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
fn select_next_stage() -> Stage {
    let formatter: OptionFormatter<Stage> = &|a| format!("Chose to {a}");
    let actions = vec![Stage::DeviceSelection, Stage::ActionSelection, Stage::Exit];
    Select::new("What would you like to do next?", actions)
        .with_formatter(formatter)
        .prompt()
        .expect("Failed to capture selection(s)")
}

/// It runs a loop that switches between two stages: device selection and action selection
pub fn run() {
    let mut stage = Stage::DeviceSelection;
    let mut selections = Vec::new();
    let mut device_count = 0;
    exec_btctl(vec!["power", "on"]).expect("Failed to power on bluetoothctl");
    loop {
        match stage {
            Stage::DeviceSelection => {
                selections = device_selection();
                device_count = selections.len();
                stage = Stage::ActionSelection;
            }
            Stage::ActionSelection => {
                let action = actions_selection(device_count);
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
                stage = select_next_stage();
            }
            Stage::Exit => break,
        }
    }
}
