// when running wasm call JS Date::now()
#![allow(unused_imports, dead_code)]   // needed for cfg switch below
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[inline]
pub fn system_time_now() -> SystemTime {
    #[cfg(target_arch = "wasm32")]
    {
        // JS milliseconds since epoch → SystemTime
        SystemTime::UNIX_EPOCH + Duration::from_millis(js_sys::Date::now() as u64)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
    }
}
