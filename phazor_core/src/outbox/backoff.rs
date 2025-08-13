//! Exponential backoff with small jitter for retry scheduling.
use std::time::Duration;
use rand::Rng;

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

/// Returns a backoff delay = base(attempts) + random jitter (0..=JITTER_MS_MAX ms).
pub fn next_backoff(attempts: u32) -> Duration {
    let base = base_secs_for(attempts);
    let jitter_ms = rand::thread_rng().gen_range(0..=JITTER_MS_MAX);
    Duration::from_secs(base) + Duration::from_millis(jitter_ms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn base_growth_and_cap() {
        // 0 -> 0, 1 -> 1, 2 -> 2, 3 -> 4, ... capped at 64
        let expected = [0u64, 1, 2, 4, 8, 16, 32, 64, 64, 64, 64];
        for (i, &exp_sec) in expected.iter().enumerate() {
            assert_eq!(base_secs_for(i as u32), exp_sec, "attempt {}", i);
        }
    }

    #[test]
    fn jitter_bounds_attempt0() {
        // attempt 0: only jitter (0..=300ms)
        for _ in 0..100 {
            let d = next_backoff(0);
            assert!(d <= Duration::from_millis(JITTER_MS_MAX));
        }
    }

    #[test]
    fn bounds_for_typical_attempt() {
        // attempt 3 -> base 4s + jitter 0..=300ms
        for _ in 0..100 {
            let d = next_backoff(3);
            assert!(d >= Duration::from_secs(4));
            assert!(d <= Duration::from_secs(4) + Duration::from_millis(JITTER_MS_MAX));
        }
    }

    #[test]
    fn bounds_for_capped_attempt() {
        // large attempts should cap at 64s base
        for _ in 0..100 {
            let d = next_backoff(50);
            assert!(d >= Duration::from_secs(64));
            assert!(d <= Duration::from_secs(64) + Duration::from_millis(JITTER_MS_MAX));
        }
    }
}
