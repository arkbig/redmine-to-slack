// Copyright © ArkBig

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
    pub fn severity(&mut self, severity: Severity) -> &mut Self {
        self.severity = severity;
        self
    }
    pub fn warning(&mut self) -> &mut Self {
        self.severity = Severity::Warning;
        self
    }
    pub fn error(&mut self) -> &mut Self {
        self.severity = Severity::Error;
        self
    }
    pub fn timestamp(&mut self, timestamp: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }
    pub fn category(&mut self, category: &'a str) -> &mut Self {
        self.category = category;
        self
    }
    pub fn message(&mut self, message: &'a str) -> &mut Self {
        self.message = message;
        self
    }
    pub fn json(&mut self, json: &'a serde_json::Value) -> &mut Self {
        self.json = json;
        self
    }
}

impl<'a> Drop for Payload<'a> {
    fn drop(&mut self) {
        println!("{}", serde_json::to_string(self).unwrap_or_default());
    }
}

#[derive(serde::Serialize)]
pub enum Severity {
    Default,
    Debug,
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

pub fn default(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message);
    payload
}

pub fn warning(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message).warning();
    payload
}

pub fn error(message: &str) -> Payload {
    let mut payload = Payload::default();
    payload.message(message).error();
    payload
}
