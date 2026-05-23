# CS2 Game State Integration Server

A Rust-based system for handling Counter-Strike 2 Game State Integration (GSI) payloads. This project infers JSON schema from GSI data and automatically generates Rust structs, providing a type-safe server for receiving and processing game state updates.

## Project Structure

This is a Rust workspace with two main crates:

### `schema_interface` Crate

A schema inference and code generation tool that analyzes Game State Integration JSON payloads.

**Key Features:**

- **Schema Inference**: Analyzes JSON files in the `./data` directory to infer the structure of GSI payloads
- **Type Detection**: Identifies and tracks all types seen for each field (bool, u64, i64, f64, string, array, object, null)
- **Example Extraction**: Collects up to 3 examples for each field to understand data patterns
- **JSON Schema Generation**: Produces a detailed `inferred_schema.json` documenting the complete structure
- **Code Generation**: Automatically generates Rust struct definitions from the inferred schema

**Main Components:**

- `lib.rs`: Core `SchemaNode` struct that recursively analyzes and stores schema information
- `generator.rs`: Converts inferred schema into Rust struct code with proper type mappings
  - Includes custom struct definitions for `RoundWins` and `Weapons` types that don't follow the default generation pattern
- `main.rs`: CLI tool that orchestrates schema inference and code generation

### `server` Crate (cs2_gsi)

An HTTP server built with Axum that receives Game State Integration POST requests from Counter-Strike 2.

**Key Features:**

- **GSI Endpoint**: Listens on `0.0.0.0:3000` for incoming POST requests from CS2 clients
- **Payload Logging**: Automatically saves each GSI payload with timestamp and request ID
- **Structured Logging**: Records payloads in JSON format with metadata for analysis

**Main Components:**

- `main.rs`: Sets up the Axum HTTP server and routing
- `handler.rs`: POST request handler that processes incoming GSI payloads
- `logger.rs`: `PayloadLogger` struct that logs payloads to disk with UUID and timestamp tracking
- `generated_structs.rs`: Auto-generated Rust structs (created by schema_interface)

## Workflow

1. **Collect Data**: GSI payloads from Counter-Strike 2 are sent to the server and automatically logged to the `./data` directory
2. **Infer Schema**: Run the schema_interface tool to analyze all payloads and generate `inferred_schema.json`
3. **Generate Code**: The same tool generates type-safe Rust structs from the inferred schema
4. **Type Safety**: The server can now use strongly-typed structs instead of generic JSON values

## Building the Project

```bash
# Build all crates
cargo build --release

# Build just the server
cargo build -p cs2_gsi --release

# Build just the schema tool
cargo build -p schema_interface --release
```

## Running the Components

### Schema Inference & Code Generation

Run the schema_interface tool to analyze collected payloads and generate Rust structs:

```bash
cargo run --bin schema_interface --release
```

This will:

1. Analyze all JSON files in `./data`
2. Generate `inferred_schema.json` with the inferred structure
3. Generate Rust struct definitions in `crates/server/src/generated_structs.rs`
4. Print a summary of the schema to stdout

### GSI Server

Start the HTTP server to receive Game State Integration payloads:

```bash
cargo run -p cs2_gsi --release
```

The server will:

1. Bind to `0.0.0.0:3000`
2. Listen for POST requests at `/`
3. Log each payload to `./data/gsi_payload_<timestamp>_<uuid>.json`
4. Return HTTP 200 OK for all requests

Configure your CS2 GSI client to POST to `http://localhost:3000/`

## Data Storage

All received GSI payloads are stored in the `./data` directory with filenames following the pattern:
`gsi_payload_YYYY-MM-DD_HHMMSS_<uuid>.json`

Each logged entry includes:

- `request_id`: UUID for unique identification
- `timestamp`: RFC3339 formatted timestamp
- `payload`: The complete GSI JSON data

## Custom Object Generation

The code generator includes special handling for certain object types that would otherwise produce verbose or awkward structures. Custom cases provide cleaner, more idiomatic Rust:

### RoundWins

Instead of generating separate fields for each round type:

```rust
struct RoundWins {
    one: String,
    two: String,
    // ... entry for each round played
}
```

We generate a unified, cleaner representation:

```rust
struct RoundWins {
    pub win_type: Vec<String>,
}
```

### Weapons

Instead of generating numbered weapon fields:

```rust
struct Weapons {
    weapon_0: Weapon0,
    // ...
    weapon_7: Weapon7,
}
struct Weapon0 {...}
// ...
struct Weapon7 {...}
```

We generate a proper collection-based API:

```rust
struct Weapons {
    pub weapons: Vec<Weapon>,
}

struct Weapon {...}
```

Additional custom cases can be added to the `handle_obj` function in [generator.rs](crates/schema_interface/src/generator.rs#L80) for other complex types.

## Configuration Files

- `Cargo.toml`: Workspace configuration
- `config.toml`: Project configuration file
- `gamestate_integration_example.cfg`: Example GSI client configuration for CS2
- `inferred_schema.json`: Generated schema file (created after running schema_interface)

## Dependencies

### Core Runtime

- **axum** (0.8.9): Async HTTP framework
- **tokio** (1.52.3): Async runtime with multi-threading support
- **serde**: Serialization/deserialization framework
- **serde_json**: JSON support

### Utilities

- **chrono** (0.4.44): DateTime handling and formatting
- **uuid** (1.23.1): UUID generation for request tracking
- **tracing-subscriber** (0.3.23): Structured logging

## Example GSI Payload Structure

The inferred schema documents the complete structure of Counter-Strike 2 GSI payloads, including:

- Game state (map, round information, player stats)
- Player inventory and equipment
- Team information
- Match metadata

See `inferred_schema.json` for the complete schema after running the inference tool.

## Architecture Highlights

- **Type Safety**: Leverages Rust's type system for compile-time correctness
- **Schema-Driven Development**: Code is generated from observed data patterns, ensuring structs match actual GSI data
- **Async-First**: Built on Tokio for high-performance async I/O
- **Automatic Tracking**: Every payload is uniquely identified and timestamped for easy debugging and analysis
