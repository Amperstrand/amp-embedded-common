//! Initialization recovery tracker for peripheral re-initialization.
//!
//! Tracks consecutive failures and triggers a re-initialization after a
//! configurable threshold. Used in CCID firmware to recover from transient
//! peripheral faults (MFRC522, smartcard reader).
//!
//! Extracted verbatim from ccid-firmware-rs 76f1e8af929130f61b19daf2c8b045a083304d79.

pub const REINIT_THRESHOLD: u8 = 3;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InitRecoveryTracker {
    consecutive_failures: u8,
    reinit_count: u32,
}

impl InitRecoveryTracker {
    pub const fn new() -> Self {
        Self {
            consecutive_failures: 0,
            reinit_count: 0,
        }
    }

    pub fn record_success(&mut self) {
        self.consecutive_failures = 0;
    }

    pub fn record_failure(&mut self) -> bool {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        if self.consecutive_failures >= REINIT_THRESHOLD {
            self.consecutive_failures = 0;
            self.reinit_count = self.reinit_count.saturating_add(1);
            true
        } else {
            false
        }
    }

    pub fn reinit_count(&self) -> u32 {
        self.reinit_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reinit_tracker_threshold() {
        let mut tracker = InitRecoveryTracker::new();
        assert_eq!(tracker.reinit_count(), 0);

        assert!(!tracker.record_failure());
        assert_eq!(tracker.reinit_count(), 0);

        assert!(!tracker.record_failure());
        assert_eq!(tracker.reinit_count(), 0);

        assert!(tracker.record_failure());
        assert_eq!(tracker.reinit_count(), 1);
    }

    #[test]
    fn test_reinit_tracker_success_resets_counter() {
        let mut tracker = InitRecoveryTracker::new();
        tracker.record_failure();
        tracker.record_failure();
        tracker.record_success();
        assert!(!tracker.record_failure());
        assert!(!tracker.record_failure());
        assert_eq!(tracker.reinit_count(), 0);
    }

    #[test]
    fn test_reinit_tracker_multiple_cycles() {
        let mut tracker = InitRecoveryTracker::new();
        for _ in 0..7 {
            tracker.record_failure();
        }
        assert_eq!(tracker.reinit_count(), 2);
    }

    #[test]
    fn test_reinit_threshold_constant() {
        assert_eq!(REINIT_THRESHOLD, 3);
    }
}
