use std::time::Instant;

use super::{LogMsg, Logger};

pub struct ConsoleLogger {
    start_time: Instant,
}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {
            start_time: Instant::now(),
        }
    }
}

impl Logger for ConsoleLogger {
    fn log_info(&self, msg: &str) {
        println!("[INFO]: {}", msg)
    }

    fn log_debug(&self, msg: &str) {
        println!("[DEBUG]: {}", msg)
    }

    fn log_warning(&self, msg: &str) {
        println!("[WARNING]: {}", msg)
    }

    fn log_error(&self, msg: &str) {
        println!("[ERROR]: {}", msg)
    }
}
