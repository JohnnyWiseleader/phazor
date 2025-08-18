#![allow(unused_imports, dead_code)]   // needed for cfg switch below
use std::time::{Duration, SystemTime};

#[inline]
pub fn system_time_now() -> SystemTime {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Date;
        SystemTime::UNIX_EPOCH + Duration::from_millis(Date::now() as u64)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
    }
}

