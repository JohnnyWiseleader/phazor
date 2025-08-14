//! Exponential backoff with small jitter for retry scheduling.
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::util::now::system_time_now;

/// Maximum exponent used for the base 2^exp seconds (caps base at 64s).
const MAX_EXPONENT: u32 = 6;
/// Jitter in milliseconds added on top of the base.
const JITTER_MS_MAX: u64 = 300;

/// Deterministic base delay in **seconds** for a given attempt count.
/// attempts: 0 -> 0s, 1 -> 1s, 2 -> 2s, 3 -> 4s, ... capped at 64s.
fn base_secs_for(attempts: u32) -> u64 {
    if attempts == 0 {
        0
    } else {
        // 2^(attempts-1), capped
        1u64 << ((attempts - 1).min(MAX_EXPONENT))
    }
}

/// A tiny "good enough" jitter using current time; no RNG crates.
/// Not cryptographic — but fine for retry staggering in a SPA.
fn jitter_ms() -> u64 {
    let now = system_time_now().duration_since(UNIX_EPOCH).unwrap();
    // Use microseconds mod range to create a quasi-random wobble
    (now.as_micros() as u64) % (JITTER_MS_MAX + 1)
}

pub fn next_backoff(attempts: u32) -> Duration {
    Duration::from_secs(base_secs_for(attempts)) + Duration::from_millis(jitter_ms())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn base_growth_and_cap() {
        let expected = [0u64,1,2,4,8,16,32,64,64,64];
        for (i,&e) in expected.iter().enumerate() {
            assert_eq!(base_secs_for(i as u32), e);
        }
    }
    #[test] fn bounds() {
        let d = next_backoff(3);
        assert!(d >= Duration::from_secs(4));
        assert!(d <= Duration::from_secs(4) + Duration::from_millis(JITTER_MS_MAX));
    }
}