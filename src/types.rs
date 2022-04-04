use ::{serde::Serialize, serde_json::Value, std::collections::HashMap};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
pub struct Item {
    data: Data,
}

#[derive(Debug, Serialize)]
pub struct Data {
    body: Body,
    level: Level,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    Message(Message),
    Trace(Trace),
}

#[derive(Debug, Serialize)]
pub struct Message {
    body: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize)]
pub struct Trace {
    frames: Vec<Frame>,
    exception: Exception,
}

#[derive(Debug, Serialize)]
pub struct Frame {
    filename: String,
    lineno: Option<u32>,
    colno: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct Exception {
    class: String,
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
