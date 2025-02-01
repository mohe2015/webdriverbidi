#![allow(clippy::all)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Extensible = HashMap<String, String>;

// -9007199254740991..9007199254740991
pub type JsInt = i64;
// 0..9007199254740991
pub type JsUint = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyParams {
    pub extensible: Extensible,
}

impl EmptyParams {
    pub fn new() -> Self {
        Self {
            extensible: Extensible::new(),
        }
    }
}
