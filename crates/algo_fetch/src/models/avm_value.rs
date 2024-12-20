/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

/// AvmValue : Represents an AVM value.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvmValue {
    /// bytes value.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    /// value type. Value `1` refers to **bytes**, value `2` refers to **uint64**
    #[serde(rename = "type")]
    pub r#type: i32,
    /// uint value.
    #[serde(rename = "uint", skip_serializing_if = "Option::is_none")]
    pub uint: Option<i32>,
}

impl AvmValue {
    /// Represents an AVM value.
    pub fn new(r#type: i32) -> AvmValue {
        AvmValue {
            bytes: None,
            r#type,
            uint: None,
        }
    }
}

