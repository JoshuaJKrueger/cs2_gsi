use schema_interface::SchemaInference;

fn main() {
    println!("Analyzing Game State Integration payloads");

    match SchemaInference::infer_from_logs("./data") {
        Ok(schema) => {
            schema.print_summary();

            let schema_json = schema.to_json_schema();

            match std::fs::write(
                "./inferred_schema.json",
                serde_json::to_string_pretty(&schema_json).unwrap(),
            ) {
                Ok(_) => println!("Schema saved to inferred_schema.json"),
                Err(e) => eprintln!("Failed to save schema: {}", e),
            }

            println!("=== All Fields ===");
            let mut sorted_fields: Vec<_> = schema.fields.iter().collect();
            sorted_fields.sort_by_key(|(path, _)| *path);

            for (path, info) in sorted_fields {
                let types: Vec<String> = info.types_seen.iter().cloned().collect();
                let type_str = types.join(" | ");
                println!(
                    "{:<50} | {} | {}/{} payloads",
                    path, type_str, info.frequency, schema.total_payloads
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to analyze schema: {}", e);
            std::process::exit(1);
        }
    }
}
