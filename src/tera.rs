use std::collections::HashMap;

use tera::Value;

use crate::BustDir;

impl tera::Function for BustDir {
    fn call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> {
        if let Some(Value::String(path)) = args.get("path") {
            let path = self.get_or_random(path);
            return Ok(Value::String(path.to_hex().to_string()));
        }
        Err(tera::Error::call_function(
            "bust",
            "BustDir requires `path`",
        ))
    }

    fn is_safe(&self) -> bool {
        false
    }
}
