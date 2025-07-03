//! Files I/O

use ix_results::errors::FileError;
use serde_json::Value;
use std::fs::File;
// use std::io::Read;
// use thiserror::Error;

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //

pub fn read_json(
    json_file: &str,
    case: &str,
    sub_case: &str,
) -> Result<Vec<String>, FileError> {
    // Read and parse JSON in one operation
    let json_value: Value = {
        let file = File::open(json_file)?;
        serde_json::from_reader(file).unwrap()
    };

    // Clean strings consistently
    let clean_string = |s: &str| s.trim_matches('"').replace('\\', "");

    // Generic value extraction with path validation
    let get_values =
        |path: &[&str], value_type: &str| -> Result<Vec<String>, FileError> {
            let mut current = &json_value;

            for key in path {
                current = current.get(key).ok_or_else(|| {
                    FileError::MissingKey(format!("Missing key: {}", key))
                })?;
            }

            match value_type {
                "array" => current
                    .as_array()
                    .ok_or_else(|| FileError::TypeMismatch("Expected array".into()))
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(clean_string))
                            .collect()
                    }),
                "object" => current
                    .as_object()
                    .ok_or_else(|| FileError::TypeMismatch("Expected object".into()))
                    .map(|obj| {
                        obj.values()
                            .filter_map(|v| v.as_str().map(clean_string))
                            .collect()
                    }),
                _ => Err(FileError::InvalidInput("Invalid value type".into())),
            }
        };

    match case {
        "tx_arbs_jito" => get_values(&["tx_arbs_jito", "tx_signature"], "array"),
        "tx_arbs_suspected" => get_values(&["tx_arbs_suspected", sub_case], "array"),
        "addresses_dex" => get_values(&["addresses_dex", sub_case], "object"),
        "tx_generic" => get_values(&["tx_generic", sub_case], "array"),
        "addresses_jito" => get_values(&["addresses_jito", sub_case], "object"),
        _ => {
            eprintln!("Unknown case: {}", case);
            Ok(Vec::new())
        }
    }
}
