use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct SchemaNode {
    pub r#type: String,
    pub fields: Option<HashMap<String, SchemaNode>>,
    pub items: Option<Box<SchemaNode>>,
}

#[derive(Deserialize)]
pub struct Schema {
    pub inferred_from_payloads: usize,
    pub root: SchemaNode,
    pub title: String,
}

pub fn generate_code(root: Value) -> Option<String> {
    let schema: Schema = serde_json::from_value(root).ok()?;

    Some(walk("GSI", &schema.root))
}

fn walk(key: &str, node: &SchemaNode) -> String {
    let name = to_rust_struct_name(key);

    match node.r#type.as_str() {
        "object" => handle_obj(name, node),
        "array" => {
            let inner = node.items.as_ref().unwrap();
            format!("Vec<{}>", walk("", inner))
        }
        "null" => "Option<()>".to_string(),
        "bool" => "bool".to_string(),
        "u64" => "u64".to_string(),
        "i64" => "i64".to_string(),
        "f64" => "f64".to_string(),
        "string" => "String".to_string(),
        _ => unreachable!("There is a multi-valued type."),
    }
}

fn sanitize_key(k: &str) -> String {
    const RESERVED: [&str; 52] = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "unsafe", "use", "where", "while", "abstract", "become", "box", "do",
        "final", "gen", "macro", "override", "priv", "try", "typeof", "unsized", "virtual",
        "yield",
    ];

    let k = k.replace('-', "_").replace(' ', "_");

    match k.as_str() {
        "_" => "underscore".to_string(),
        k if RESERVED.contains(&k) => format!("r#{}", k),
        _ => k.to_string(),
    }
}

fn to_rust_struct_name(k: &str) -> String {
    sanitize_key(k)
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn handle_obj(name: String, node: &SchemaNode) -> String {
    let fields = node.fields.as_ref().unwrap();
    match name.as_str() {
        "RoundWins" => "struct RoundWins {\n\tpub win_type: Vec<String>,\n}\n".to_string(),
        "Weapons" => {
            let weapon = fields.values().next().unwrap();
            let weapon_struct = walk("Weapon", weapon);
            format!(
                "struct Weapons {{\n\tpub weapons: Vec<Weapon>,\n}}\n{}",
                weapon_struct
            )
        }
        // "TeamCt" => {
        //     let mut lines = Vec::new();
        //     for (k, v) in fields {
        //         lines.push(format!("\tpub {}: {}", sanitize_key(k), walk(k, v)));
        //     }
        //     format!(
        //         "struct Team {{\n\tpub side: Sides,\n{}\n}}\nenum Sides {{\n\tCT,\n\tT,\n}}\n",
        //         lines.join(",\n")
        //     )
        // }
        // "TeamT" => String::new(),
        _ => {
            let mut lines = Vec::new();
            let mut nested_out = String::new();
            for (k, v) in fields {
                let is_obj = v.r#type.as_str() == "object";
                let inner = if is_obj {
                    nested_out.push_str(&walk(k, v));
                    to_rust_struct_name(k)
                } else {
                    walk(k, v)
                };

                lines.push(format!("\tpub {}: {}", sanitize_key(k), inner));
            }
            format!(
                "struct {} {{\n{}\n}}\n{}",
                name,
                lines.join(",\n"),
                nested_out
            )
        }
    }
}
