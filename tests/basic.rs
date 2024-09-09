#[cfg(test)]
mod tests {
    use super::*;
    use oxide_frame::DataFrame;
    use oxide_frame::DataValue;

    // Test case to verify that adding a column works as expected.
    #[test]
    fn test_add_column() {
        // Create an empty DataFrame.
        let mut df = DataFrame::new();

        // Add a column called "Age" with integer values.
        df.add_column(
            "Age",
            vec![DataValue::Int(25), DataValue::Int(30), DataValue::Int(22)],
        );

        // Verify that the column count is now 1.
        assert_eq!(df.columns.len(), 1);

        // Verify the correct shape (3 rows, 1 column).
        assert_eq!(df.shape(), (3, 1));
    }
}
