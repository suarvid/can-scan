mod simple_can_frame;

use crate::simple_can_frame::SimpleCanFrame;

use serde_yaml::{self};
use socketcan::{BlockingCan, CanSocket, Socket};

use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Incorrect number of arguments!");
        eprintln!("Usage: {} <can-interface> <yaml-file> <n-frames>", args[0]);
        panic!("Example: {} can0 msgs.yaml 1000", args[0]);
    }

    let ifname = &args[1];
    let file_path = &args[2];
    let frame_count = match args[3].parse() {
        Ok(count) => count,
        Err(_) => panic!("Failed to parse {} into a number of frames!", args[3]),
    };

    let expected_frames = deserialize_expected_frames(file_path);

    let received_frames = receive_frames(ifname, frame_count);

    for frame in expected_frames {
        if !received_frames.contains(&frame) {
            println!("CAN frame {:#?} was not received as expected.", frame);
        }
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
