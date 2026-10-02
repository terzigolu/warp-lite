
use serde_json::json;

use super::{
    settings_schema_json, strip_empty_enum_entries, strip_numeric_metadata,
};

#[test]
fn strips_numeric_metadata_recursively() {
    let mut schema = json!({
        "type": "object",
        "properties": {
            "count": {
                "type": "integer",
                "minimum": 0,
                "maximum": 255,
                "format": "uint8"
            }
        }
    });

    strip_numeric_metadata(&mut schema);

    assert_eq!(
        schema,
        json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "integer"
                }
            }
        })
    );
}

#[test]
fn strips_empty_enum_entries() {
    let mut schema = json!({
        "oneOf": [
            {
                "enum": [],
                "type": "string"
            },
            {
                "const": "kept"
            }
        ]
    });

    strip_empty_enum_entries(&mut schema);

    assert_eq!(
        schema,
        json!({
            "oneOf": [
                {
                    "const": "kept"
                }
            ]
        })
    );
}

#[test]
fn generates_a_settings_schema() {
    let schema = settings_schema_json(|_| false).unwrap();
    let schema: serde_json::Value = serde_json::from_str(&schema).unwrap();

    assert_eq!(schema["title"], "Warp Settings");
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"].is_object());
}

#[test]
fn writes_schema_atomically_to_a_new_directory() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested/settings.json");
    super::write_atomically(&path, b"{\"version\":1}").unwrap();
    super::write_atomically(&path, b"{\"version\":2}").unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), "{\"version\":2}");
}
