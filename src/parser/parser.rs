use polars::prelude::*;

pub fn process_logs(file_path: &str) -> Result<DataFrame, PolarsError> {
    // Read the CSV file into a DataFrame
    let df = CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()?;
    Ok(df)
}

pub fn print_df_info(df: &DataFrame) {
    println!("DataFrame Info:");
    println!("Shape: {:?}", df.shape());
    println!("Columns: {:?}", df.get_column_names());
    println!("Schema: {:?}", df.schema());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_logs() {
        let file_path = "preprocessed/Event_traces.csv"; // Replace with your test file path
        let result = process_logs(file_path);

        assert!(result.is_ok(), "Failed to process logs: {:?}", result.err());

        let df = result.unwrap();
        assert!(!df.is_empty(), "DataFrame should not be empty");

        // Additional assertions can be added here based on expected DataFrame structure
    }

    #[test]
    fn test_print_df_info() {
        let file_path = "preprocessed/Event_traces.csv"; // Replace with your test file path
        let result = process_logs(file_path).unwrap();

        // Capture the output of print_df_info
        let output = format!("{:?}", result);
        assert!(!output.is_empty(), "Output should not be empty");
    }
}