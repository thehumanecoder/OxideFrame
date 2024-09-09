# OxideFrame

`OxideFrame` is a Rust library inspired by Python's `pandas`, designed for efficient data manipulation and analysis. It provides an easy-to-use structure for handling tabular data, such as data frames, along with basic operations like adding columns, filtering rows, and data serialization.

## Features

- Create and manipulate data frames.
- Support for multiple data types (`Int`, `Float`, `Str`, `Bool`, `Null`).
- Filtering rows based on column values.
- JSON and CSV input/output.

## Installation

Add `oxide_frame` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
oxide_frame = "0.1.0"
```

## Usage

### Creating a DataFrame

```rust
use oxide_frame::{DataFrame, DataValue};

let mut df = DataFrame::new();
df.add_column("Name", vec![
    DataValue::Str("Alice".to_string()),
    DataValue::Str("Bob".to_string())
]);
df.add_column("Age", vec![
    DataValue::Int(25),
    DataValue::Int(30)
]);
```

### Filtering Rows

```rust
let filtered_df = df.filter("Age", |val| {
    if let DataValue::Int(age) = val {
        *age > 25
    } else {
        false
    }
});
```

### JSON Serialization

```rust
use oxide_frame::io::json;
let json_data = json::save_to_json(&df);
```

## Roadmap

- Additional data aggregation methods (sum, mean, etc.).
- Support for missing data (Null handling).
- Integration with more data formats (e.g., Parquet, Excel).

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for bug reports, feature requests, or suggestions.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
