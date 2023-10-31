// Copyright © ArkBig

static LOG_LEVEL: std::sync::OnceLock<Severity> = std::sync::OnceLock::new();
static LOG_LEVEL_DEFAULT: Severity = Severity::Default;

pub fn set_level(level: Severity) {
    let _ = LOG_LEVEL.set(level);
}

/// Log payload
///
/// Log writing at drop time.(i.e. This should be short-lived.)
///
/// # Examples:
/// ```rust
/// # use zenn_news_lib::log;
/// log::default("Message").category("Category");
/// log::default("Message").category("Category2").warning();
/// log::warning("Message").category("Category3");
#[derive(serde::Serialize)]
pub struct Payload<'a> {
    severity: Severity,
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: chrono::DateTime<chrono::Utc>,
    category: &'a str,
    message: &'a str,
    #[serde(skip_serializing_if = "is_null_value")]
    json: &'a serde_json::Value,
}

fn is_null_value(value: &serde_json::Value) -> bool {
    value.is_null()
}

impl<'a> Payload<'a> {
    pub fn default() -> Self {
        Payload {
            severity: Severity::Default,
            timestamp: chrono::Utc::now(),
            category: "None",
            message: "",
            json: &serde_json::Value::Null,
        }
    }
    #[allow(dead_code)]
    pub fn severity(&mut self, severity: Severity) -> &mut Self {
        self.severity = severity;
        self
    }
    #[allow(dead_code)]
    pub fn debug(&mut self) -> &mut Self {
        self.severity = Severity::Debug;
        self
    }
    #[allow(dead_code)]
    pub fn warning(&mut self) -> &mut Self {
        self.severity = Severity::Warning;
        self
    }
    #[allow(dead_code)]
    pub fn error(&mut self) -> &mut Self {
        self.severity = Severity::Error;
        self
    }
    #[allow(dead_code)]
    pub fn timestamp(&mut self, timestamp: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }
    pub fn category(&mut self, category: &'a str) -> &mut Self {
        self.category = category;
        self
    }
    #[allow(dead_code)]
    pub fn message(&mut self, message: &'a str) -> &mut Self {
        self.message = message;
        self
    }
    #[allow(dead_code)]
    pub fn json(&mut self, json: &'a serde_json::Value) -> &mut Self {
        self.json = json;
        self
    }
}

impl<'a> Drop for Payload<'a> {
    fn drop(&mut self) {
        if LOG_LEVEL.get().unwrap_or_else(|| &LOG_LEVEL_DEFAULT) <= &self.severity {
            println!("{}", serde_json::to_string(self).unwrap_or_default());
        }
    }
}

#[derive(serde::Serialize, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum Severity {
    Debug,
    Default,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Severity::Default => write!(f, "DEFAULT"),
            Severity::Debug => write!(f, "DEBUG"),
            Severity::Info => write!(f, "INFO"),
            Severity::Notice => write!(f, "NOTICE"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::Alert => write!(f, "ALERT"),
            Severity::Emergency => write!(f, "EMERGENCY"),
        }
    }
}

#[allow(dead_code)]
pub fn default(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message);
    payload
}

#[allow(dead_code)]
pub fn debug(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message).debug();
    payload
}

#[allow(dead_code)]
pub fn warning(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message).warning();
    payload
}

#[allow(dead_code)]
pub fn error(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message).error();
    payload
}
