use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CrocEvent {
    /// Parsed output for sending files
    TransferOutput(CrocTransferOutput),
    /// Parsed output for hashing files
    HashOutput(CrocHashOutput),
    /// A code has been generated for the transfer session
    CodeGenerated(String),
    /// Finished sending or receiving files
    Done,
    Unknown(String),
    /// An error occurred during the transfer process
    EOF,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CrocTransferOutput {
    pub progress: u8,
    pub total_size: usize,
    pub total_sent: usize,
    pub speed: usize,
    pub time_spent: Option<usize>,
    pub time_remaining: Option<usize>,
    pub filename: String,
    pub raw_message: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CrocHashOutput {
    pub progress: u8,
    pub speed: usize,
    pub time_spent: usize,
    pub time_remaining: usize,
    pub filename: String,
    pub raw_message: String,
}

impl Display for CrocEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrocEvent::TransferOutput(_) => write!(f, "TransferOutput"),
            CrocEvent::HashOutput(_) => write!(f, "HashOutput"),
            CrocEvent::CodeGenerated(_) => write!(f, "CodeGenerated"),
            CrocEvent::Done => write!(f, "Done"),
            CrocEvent::Unknown(line) => write!(f, "Unknown: {}", line),
            CrocEvent::EOF => write!(f, "EOF"),
        }
    }
}
