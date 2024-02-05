use core::fmt;
use serde::Deserialize;
use socketcan::{CanFrame, EmbeddedFrame, Frame};
use std::hash::Hash;

/// Represents the most basic components of a CAN frame.
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct SimpleCanFrame {
    /// Optional name of a frame, such as if interpreted in a DBC file
    name: Option<String>,
    /// The ID of the CAN frame
    id: u32,
    /// The DLC of the CAN frame
    dlc: usize,
    /// The data field of the CAN frame
    data: Vec<u8>,
}

impl SimpleCanFrame {
    pub fn from_can_frame(from_frame: CanFrame) -> Self {
        SimpleCanFrame {
            name: None,
            id: from_frame.raw_id(),
            dlc: from_frame.dlc(),
            data: from_frame.data().to_vec(),
        }
    }
}

impl fmt::Display for SimpleCanFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Id: {}\tdlc: {}\tdata: {:#?}",
            self.id, self.dlc, self.data
        )
    }
}

/// Implementation of the Hash trait for SimpleCanFrame
/// Note that the hash is not calculated based on the frame name,
/// as one might not always be available, or may differ depending on
/// the DBC file used to interpret frames.
impl Hash for SimpleCanFrame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.dlc.hash(state);
        self.data.hash(state);
    }
}
