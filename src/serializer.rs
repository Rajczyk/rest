use std::collections::HashMap;
use serde_json::value::{ToJson};


pub trait ToJsonString {
    /// Converts the value of `self` to an instance of JSON
    fn tojson(&self) -> String;
}

impl ToJsonString for HashMap<String,String>
{
    fn tojson(&self) -> String {
        self.to_json().to_string()
    }
}