mod simple_can_frame;

use crate::simple_can_frame::SimpleCanFrame;

use serde_yaml::{self};
use socketcan::{BlockingCan, CanSocket, Socket};

use std::{collections::HashSet, env, fs, process};

fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|error| {
        eprintln!("Failed to parse given arguments: {error}");
        process::exit(1);
    });

    let expected_frames = deserialize_expected_frames(&config.expected_msgs_file);

    let received_frames = receive_frames(&config.interface_name, config.capture_frame_count);

    for frame in expected_frames {
        if !received_frames.contains(&frame) {
            println!("CAN frame {:#?} was not received as expected.", frame);
        }
    }
}
/// Represents the configuration of the program, based on the given
/// command-line arguments
struct Config {
    interface_name: String,
    expected_msgs_file: String,
    capture_frame_count: usize,
}

impl Config {
    fn from_args(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // Skip name of function

        let if_name = match args.next() {
            Some(if_name) => if_name,
            None => return Err("No CAN interface name specified."),
        };

        let msgs_file = match args.next() {
            Some(msgs_file) => msgs_file,
            None => return Err("No file containing expected messages given."),
        };

        let frame_count = match args.next() {
            Some(count) => {
                let count: usize = count.parse().expect("Invalid number of frames to capture.");
                count
            }
            None => return Err(""),
        };

        Ok(Config {
            interface_name: if_name,
            expected_msgs_file: msgs_file,
            capture_frame_count: frame_count,
        })
    }
}

/// Opens the given YAML file, creating a vector of CAN frames we expect
/// to receive from its contents.
///
/// # Arguments
///
/// *  `file_path` - Path to the YAML file
fn deserialize_expected_frames(file_path: &String) -> Vec<SimpleCanFrame> {
    let expected_f = fs::File::open(file_path)
        .expect("Failed to open the given yaml-file of expected messages.");
    let expected_msgs: Vec<SimpleCanFrame> = serde_yaml::from_reader(expected_f)
        .expect("Failed to deserialize the expected message file.");
    expected_msgs
}

/// Opens the given CAN interface, receives the given number of frames,
/// returning a set of unique frames received.
///
/// # Arguments
///
/// *  `ifname` - Name of the CAN interface, "can0" for example
/// *  `frame_count` - The number of frames to receive
fn receive_frames(ifname: &String, frame_count: usize) -> HashSet<SimpleCanFrame> {
    let mut frame_set = HashSet::new();
    let mut rx_sock = CanSocket::open(ifname)
        .expect("Failed to open a CAN socket using the given interface name.");

    let mut running_frame_count = 0;

    while running_frame_count < frame_count {
        if let Ok(rx_frame) = rx_sock.receive() {
            let simplified = SimpleCanFrame::from_can_frame(rx_frame);
            frame_set.insert(simplified);

            running_frame_count += 1;
        } else {
            eprintln!("Error occured when receiving a frame, continuing...")
        }
    }

    frame_set
}
