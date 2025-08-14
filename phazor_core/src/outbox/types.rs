use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::util::now::system_time_now;

// simple per-process counter to avoid collisions within the same timestamp
static COUNTER: AtomicU64 = AtomicU64::new(0);

fn make_id_from(kind: &MessageKind, created_at: SystemTime) -> Uuid {
    let ts = created_at.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let c = COUNTER.fetch_add(1, Ordering::Relaxed);
    // Use the collection/topic to make the name unique and meaningful
    let name = match kind {
        MessageKind::Create { collection }
        | MessageKind::Update { collection }
        | MessageKind::Delete { collection } => format!("{collection}:{ts}:{c}"),
        MessageKind::Custom { topic } => format!("{topic}:{ts}:{c}"),
    };
    // Deterministic UUIDv5 using the URL namespace (any of DNS/URL/OID/X500 is fine)
    Uuid::new_v5(&Uuid::NAMESPACE_URL, name.as_bytes())
}

mod system_time_ms {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(t: &SystemTime, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dur = t
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        s.serialize_u64(dur.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = u64::deserialize(d)?;
        Ok(SystemTime::UNIX_EPOCH + Duration::from_millis(ms))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageKind {
    Create { collection: String },
    Update { collection: String },
    Delete { collection: String },
    Custom { topic: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub kind: MessageKind,
    /// Typically JSON payload; keep it generic so Python future can reuse.
    pub payload: serde_json::Value,
    #[serde(with = "system_time_ms")]
    pub created_at: SystemTime,
}

impl Message {
    pub fn new(kind: MessageKind, payload: serde_json::Value) -> Self {
        let created_at = system_time_now();
        let id = make_id_from(&kind, created_at);
        Self { id, kind, payload, created_at }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    InFlight,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Envelope {
    pub msg: Message,
    pub state: DeliveryState,
    pub attempts: u32,
    #[serde(with = "system_time_ms")]
    pub next_attempt_after: SystemTime,
    pub last_error: Option<String>,
}

impl Envelope {
    pub fn new(msg: Message) -> Self {
        Self {
            msg,
            state: DeliveryState::Pending,
            attempts: 0,
            next_attempt_after: SystemTime::UNIX_EPOCH,
            last_error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string};
    use uuid::Uuid;

    #[test]
    fn envelope_json_roundtrip() {
        // Use fixed timestamps to avoid sub-ms truncation surprises.
        let fixed_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1_725_000_000);

        let msg = Message {
            id: Uuid::new_v5(&Uuid::NAMESPACE_URL, b"envelope_json_roundtrip"),
            kind: MessageKind::Create {
                collection: "todos".into(),
            },
            payload: serde_json::json!({"title": "Buy hay bales", "done": false}),
            created_at: fixed_time,
        };

        let env = Envelope {
            msg,
            state: DeliveryState::Pending,
            attempts: 0,
            next_attempt_after: SystemTime::UNIX_EPOCH, // due now
            last_error: None,
        };

        let json = to_string(&env).expect("serialize");
        let back: Envelope = from_str(&json).expect("deserialize");

        assert_eq!(env, back, "envelope should round-trip via JSON intact");
        // Spot-check a few important fields explicitly:
        assert_eq!(back.msg.created_at, fixed_time);
        assert!(matches!(back.state, DeliveryState::Pending));
        assert_eq!(back.msg.payload["title"], "Buy hay bales");
    }

    #[test]
    fn message_new_sets_fields() {
        let m = Message::new(
            MessageKind::Custom {
                topic: "ping".into(),
            },
            serde_json::json!({"ts": 1}),
        );
        // UUID should be non-nil
        assert_ne!(m.id, Uuid::nil());
        // created_at should be >= UNIX_EPOCH
        assert!(m.created_at.duration_since(SystemTime::UNIX_EPOCH).is_ok());
    }
}
