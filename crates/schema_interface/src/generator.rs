use serde_json::Value;

pub fn generate_code(root: Value) -> String {
    let fields = root
        .get("root")
        .expect("Couldn't find fields in inferred_schema.json")
        .as_object()
        .expect("Expected fields to be an object.");
    let code = String::new();
    for (k, v) in fields {}
    code
}
