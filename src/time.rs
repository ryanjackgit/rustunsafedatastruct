use std::time::Duration;
use std::time::Instant;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
pub struct TimerWatch {
    current:SystemTime,
}

impl TimerWatch {
    pub fn new() -> Self {
        TimerWatch {
            current: SystemTime::now(),
        }
    }
}

impl TimerWatch {
   pub fn passed(&self) -> u128 {
        SystemTime::now().duration_since(self.current).unwrap().as_millis()
    }
}