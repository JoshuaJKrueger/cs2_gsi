use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub path: String,
    pub types_seen: HashSet<String>,
    pub examples: Vec<Value>,
    pub frequency: usize,
}

pub struct SchemaInference {
    pub fields: HashMap<String, FieldInfo>,
    pub total_payloads: usize,
}

impl SchemaInference {
    pub fn new() -> Self {
        SchemaInference {
            fields: HashMap::new(),
            total_payloads: 0,
        }
    }

    pub fn infer_from_logs(data_dir: &str) -> Result<Self, String> {
        let mut schema = SchemaInference::new();

        let entries =
            fs::read_dir(data_dir).map_err(|e| format!("Failed to read data dir: {}", e))?;

        let mut payloads = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("Error reading entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

                if let Ok(json_value) = serde_json::from_str::<Value>(&content) {
                    payloads.push(json_value);
                }
            }
        }

        println!("Found {} payload files\n", payloads.len());

        for wrapper in payloads.iter() {
            schema.total_payloads += 1;

            if let Some(payload) = wrapper.get("payload") {
                schema.collect_fields("", payload, true);

                if let Some(previously) = wrapper.get("payload").and_then(|p| p.get("previously")) {
                    if previously.is_object() {
                        schema.collect_fields("", previously, false);
                    }
                }

                if let Some(added) = wrapper.get("payload").and_then(|p| p.get("added")) {
                    if added.is_object() {
                        schema.collect_fields("", added, true);
                    }
                }
            }
        }

        Ok(schema)
    }

    fn collect_fields(&mut self, prefix: &str, value: &Value, _is_current: bool) {
        match value {
            Value::Object(map) => {
                for (key, val) in map.iter() {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };

                    if path == "request_id" || path == "timestamp" {
                        continue;
                    }

                    match val {
                        Value::Object(_) => {
                            self.collect_fields(&path, val, _is_current);
                        }
                        _ => {
                            self.record_field(&path, val);
                        }
                    }
                }
            }
            _ => {
                if !prefix.is_empty() {
                    self.record_field(prefix, value);
                }
            }
        }
    }

    fn record_field(&mut self, path: &str, value: &Value) {
        let type_name = match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => {
                if value.is_i64() {
                    "integer"
                } else {
                    "number"
                }
            }
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };

        let entry = self.fields.entry(path.to_string()).or_insert(FieldInfo {
            path: path.to_string(),
            types_seen: HashSet::new(),
            examples: Vec::new(),
            frequency: 0,
        });

        entry.types_seen.insert(type_name.to_string());
        entry.frequency += 1;

        if entry.examples.len() < 3 {
            entry.examples.push(value.clone());
        }
    }

    pub fn to_json_schema(&self) -> Value {
        let mut schema = json!({
            "title": "CS:GO Game State Integration Schema",
            "inferred_from_payloads": self.total_payloads,
            "fields": {}
        });

        let fields_obj = schema["fields"].as_object_mut().unwrap();

        let mut sorted_paths: Vec<_> = self.fields.iter().collect();
        sorted_paths.sort_by_key(|(path, _)| *path);

        for (path, info) in sorted_paths {
            let types: Vec<String> = info.types_seen.iter().cloned().collect();
            let type_str = if types.len() == 1 {
                types[0].clone()
            } else {
                format!("{} types", types.len())
            };

            fields_obj.insert(
                path.clone(),
                json!({
                    "type": type_str,
                    "seen_in": format!("{}/{}", info.frequency, self.total_payloads),
                    "examples": info.examples.iter().take(3).collect::<Vec<_>>()
                }),
            );
        }

        schema
    }

    pub fn print_summary(&self) {
        println!("\n=== Schema Inference Summary ===");
        println!("Total payloads: {}", self.total_payloads);
        println!("Unique fields: {}\n", self.fields.len());

        let mut top_level: HashSet<&str> = HashSet::new();
        for path in self.fields.keys() {
            if let Some(first) = path.split('.').next() {
                top_level.insert(first);
            }
        }

        let mut components: Vec<&str> = top_level.into_iter().collect();
        components.sort();

        println!("Components:");
        for component in components {
            let count = self
                .fields
                .iter()
                .filter(|(k, _)| {
                    k.starts_with(&format!("{}.", component)) || k.as_str() == component
                })
                .count();
            println!("  - {}: {} fields", component, count);
        }
    }
}
