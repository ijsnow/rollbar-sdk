use ::{serde::Serialize, serde_json::Value, std::collections::HashMap};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Item {
    data: Data,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Data {
    body: Body,
    level: Level,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "c", repr(u8))]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl<Str: AsRef<str>> From<Str> for Level {
    fn from(s: Str) -> Self {
        use Level::*;

        match s.as_ref() {
            "debug" => Debug,
            "info" => Info,
            "warning" => Warning,
            "critical" => Critical,
            _ => Error,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    Message(Message),
    Trace(Trace),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Message {
    body: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Trace {
    frames: Vec<Frame>,
    exception: Exception,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Frame {
    filename: String,
    lineno: Option<u32>,
    colno: Option<u32>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Exception {
    class: String,
}

impl<AsStr: Into<String>> From<(Level, AsStr, HashMap<String, Value>)> for Item {
    fn from((level, message, extra): (Level, AsStr, HashMap<String, Value>)) -> Self {
        let message = Message {
            body: message.into(),
            extra,
        };

        Self {
            data: Data {
                body: Body::Message(message),
                level,
                context: None,
                language: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_item_properly_formats() {
        use serde_json::{json, to_value};

        let input = Item {
            data: Data {
                body: Body::Message(Message {
                    body: "this is a test".into(),
                    extra: {
                        let mut extra = HashMap::new();
                        extra.insert("extra_data".into(), "right here".into());
                        extra
                    },
                }),
                level: Level::Info,
                language: None,
                context: None,
            },
        };

        let got = to_value(input).unwrap();

        let want = json!({
            "data": {
                "body": {
                    "message": {
                        "body": "this is a test",
                        "extra_data": "right here"
                    }
                },
                "level": "info"
            }
        });

        assert_eq!(got, want);
    }
}
