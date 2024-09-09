use crate::data_value::DataValue;
use std::collections::HashMap;

// The `DataFrame` struct represents a 2D data structure similar to a table, where columns are
// stored as a `HashMap`. The key is the column name, and the value is a `Vec` of `DataValue`.
pub struct DataFrame {
    pub columns: HashMap<String, Vec<DataValue>>, // Columns store data values under column names
}

impl DataFrame {
    // Constructs a new, empty `DataFrame`.
    pub fn new() -> Self {
        DataFrame {
            columns: HashMap::new(),
        }
    }

    // Adds a new column to the `DataFrame`. Takes the column name as a string slice and a vector
    // of `DataValue`s to be inserted under that column.
    pub fn add_column(&mut self, name: &str, values: Vec<DataValue>) {
        self.columns.insert(name.to_string(), values);
    }

    // Retrieves the shape of the `DataFrame` as a tuple (rows, columns).
    // Rows are determined by the length of any column (assuming equal lengths).
    pub fn shape(&self) -> (usize, usize) {
        // Get the number of rows by checking the length of the first column
        let rows = self.columns.values().next().map_or(0, |col| col.len());
        let cols = self.columns.len(); // The number of columns
        (rows, cols)
    }

    // Retrieves a reference to a column by its name. If the column exists, it returns Some(reference).
    pub fn get_column(&self, name: &str) -> Option<&Vec<DataValue>> {
        self.columns.get(name)
    }

    // Filters the `DataFrame` based on a condition (closure). The closure takes a reference to a
    // value in the column specified by `column_name` and returns a boolean.
    pub fn filter<F>(&self, column_name: &str, predicate: F) -> DataFrame
    where
        F: Fn(&DataValue) -> bool, // `predicate` is the filtering condition
    {
        let mut filtered = DataFrame::new();

        // If the target column exists, iterate over all other columns and apply the filter.
        if let Some(column) = self.get_column(column_name) {
            for (col_name, values) in &self.columns {
                let filtered_col: Vec<DataValue> = values
                    .iter()
                    .enumerate()
                    .filter_map(|(i, val)| {
                        if predicate(&column[i]) {
                            Some(val.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                filtered.add_column(col_name, filtered_col);
            }
        }

        filtered
    }
}
