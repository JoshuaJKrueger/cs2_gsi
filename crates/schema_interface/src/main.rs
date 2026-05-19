use schema_interface::SchemaInference;
use schema_interface::generator;

fn gen_schema() {
    println!("Analyzing Game State Integration payloads");

    match SchemaInference::infer("./data") {
        Ok(schema) => {
            schema.print_summary();

            let schema_json = schema.to_json_schema();

            match std::fs::write(
                "./inferred_schema.json",
                serde_json::to_string_pretty(&schema_json).expect("didn't receive proper json"),
            ) {
                Ok(_) => println!("Schema saved to inferred_schema.json"),
                Err(e) => eprintln!("Failed to save schema: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Failed to analyze schema: {}", e);
            std::process::exit(1);
        }
    }
}

fn gen_code() {
    println!("Generating Rust structs from schema\n");

    let schema_content = match std::fs::read_to_string("./inferred_schema.json") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to analyze schema: {}", e);
            std::process::exit(1);
        }
    };
    let schema: serde_json::Value = match serde_json::from_str(&schema_content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to parse schema: {}", e);
            std::process::exit(1);
        }
    };

    let generated_code = generator::generate_code(schema);
    let output_path = "./crates/server/src/generated_structs.rs";
    match std::fs::write(output_path, &generated_code) {
        Ok(_) => println!("Saved to {}", output_path),
        Err(e) => eprintln!("Warning: Could not save to file: {}", e),
    }
}

fn main() {
    gen_schema();
    gen_code();
}
