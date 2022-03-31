use ::{
    serde::Serialize,
    serde_json::Value,
    std::{collections::HashMap, panic::PanicInfo},
};

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
#[wasm_bindgen]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl From<log::Level> for Level {
    fn from(level: log::Level) -> Level {
        use log::Level::*;
        match level {
            Trace => Level::Debug,
            Debug => Level::Debug,
            Info => Level::Info,
            Warn => Level::Warning,
            Error => Level::Error,
        }
    }
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

impl<'a> From<&log::Record<'a>> for Item {
    fn from(record: &log::Record<'a>) -> Self {
        let mut extra = HashMap::new();

        extra.insert("target".into(), record.target().into());

        let mut context: Vec<String> = vec![];

        if let Some(file) = record.file() {
            context.push(file.into());
            extra.insert("file".into(), file.into());
        }

        if let Some(module_path) = record.module_path() {
            context.push(module_path.into());
            extra.insert("module_path".into(), module_path.into());
        }

        if let Some(line) = record.line() {
            context.push(line.to_string());
            extra.insert("lineno".into(), line.into());
        }

        let message = Message {
            body: format!("{}", record.args()),
            extra,
        };

        Self {
            data: Data {
                body: Body::Message(message),
                level: Level::from(record.level()),
                context: Some(context.join("::")),
                language: Some("rust".into()),
            },
        }
    }
}
impl From<(Level, &str, HashMap<String, Value>)> for Item {
    fn from((level, message, extra): (Level, &str, HashMap<String, Value>)) -> Self {
        let mut context: Vec<String> = vec![];

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

impl<'a> From<&PanicInfo<'a>> for Item {
    fn from(info: &PanicInfo<'a>) -> Self {
        let frame = info.location().map(|location| Frame {
            filename: location.file().into(),
            lineno: Some(location.line()),
            colno: Some(location.column()),
        });

        let message = info
            .payload()
            .downcast_ref::<&str>()
            .unwrap_or(&"panic!(...)");

        Item {
            data: Data {
                body: Body::Trace(Trace {
                    frames: frame.into_iter().collect(),
                    exception: Exception {
                        class: message.to_string(),
                    },
                }),
                level: Level::Critical,
                language: Some("rust".into()),
                context: None,
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
