// This enum defines the possible data types that a `DataFrame` can store.
// It supports integers, floats, strings, booleans, and null values.
#[derive(Debug, Clone)]
pub enum DataValue {
    Int(i32),    // Represents integer values.
    BigInt(i64), // Represents long integer values.
    Float(f64),  // Represents floating-point numbers.
    Str(String), // Represents string values.
    Bool(bool),  // Represents boolean values (true/false).
    Null,        // Represents null or missing values.
}
