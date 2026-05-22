use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;

pub mod generator;

#[derive(Default)]
pub struct SchemaNode {
    pub types_seen: HashSet<String>,
    pub examples: Vec<Value>,
    pub fields: HashMap<String, SchemaNode>,
    pub items: Option<Box<SchemaNode>>,
}

impl SchemaNode {
    fn record(&mut self, value: &Value) {
        let type_name = match value {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Number(n) => {
                if n.is_u64() {
                    "u64"
                } else if n.is_i64() {
                    "i64"
                } else {
                    "f64"
                }
            }
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };

        self.types_seen.insert(type_name.to_string());

        if self.examples.len() < 3
            && !matches!(value, Value::Object(_) | Value::Array(_))
            && !self.examples.contains(value)
        {
            self.examples.push(value.clone());
        }
    }

    fn collect(node: &mut SchemaNode, value: &Value) {
        node.record(value);

        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    if key == "previously" || key == "added" {
                        continue;
                    }
                    let child = node.fields.entry(key.clone()).or_default();

                    Self::collect(child, val);
                }
            }
            Value::Array(vec) => {
                let item = node.items.get_or_insert_default();

                for val in vec {
                    Self::collect(item, val);
                }
            }
            _ => (),
        }
    }

    fn to_json(&self, total: usize) -> Value {
        let mut obj = serde_json::Map::new();

        let types: Vec<_> = self.types_seen.iter().cloned().collect();

        obj.insert("type".to_string(), json!(types.join(" | ")));
        obj.insert(
            "examples".to_string(),
            json!(self.examples.iter().take(3).collect::<Vec<_>>()),
        );

        if !self.fields.is_empty() {
            let mut fields = serde_json::Map::new();
            let keys: Vec<_> = self.fields.keys().collect();

            for k in keys {
                fields.insert(k.clone(), self.fields[k].to_json(total));
            }

            obj.insert("fields".into(), json!(fields));
        }

        if let Some(items) = &self.items {
            obj.insert("items".into(), items.to_json(total));
        }

        json!(obj)
    }

    fn print_node_summary(&self, node: &SchemaNode, indent: usize) {
        let pad = "  ".repeat(indent);
        let keys: Vec<_> = node.fields.keys().collect();

        for key in keys {
            let child = &node.fields[key];
            let types: Vec<_> = child.types_seen.iter().cloned().collect();

            println!("{}- {} ({})", pad, key, types.join(" | "),);

            self.print_node_summary(child, indent + 1);
        }

        if node.items.is_some() {
            println!("{}- [array items]", pad);
            self.print_node_summary(node.items.as_ref().unwrap(), indent + 1);
        }
    }
}

#[derive(Default)]
pub struct SchemaInference {
    pub root: SchemaNode,
    pub total_payloads: usize,
}

impl SchemaInference {
    pub fn infer(data_dir: &str) -> Result<Self, String> {
        let mut schema = SchemaInference::default();

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
                SchemaNode::collect(&mut schema.root, payload);
            }
        }

        Ok(schema)
    }

    pub fn to_json_schema(&self) -> Value {
        json!({
            "title": "CS2 Game State Integration Schema",
            "inferred_from_payloads": self.total_payloads,
            "root": self.root.to_json(self.total_payloads)
        })
    }

    pub fn print_summary(&self) {
        println!("=== Schema Inference Summary ===");
        println!("Total payloads: {}", self.total_payloads);
        println!();

        println!("Root fields:");
        self.root.print_node_summary(&self.root, 1);
    }
}
