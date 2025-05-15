use polars::prelude::*;
use rayon::prelude::*;
use crate::parser::worker_pool;

pub fn process_logs(file_path: &str) -> Result<DataFrame, PolarsError> {
    // Read the CSV file into a DataFrame
    let df = CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()?;
    Ok(df)
}

pub fn multi_files(file_paths: Vec<&str>) -> Result<Vec<DataFrame>, PolarsError> {
    // Get file paths from the command line arguments
    let file_paths: Vec<String> = file_paths.iter().map(|s| s.to_string()).collect();
    let num_files = file_paths.len();

    // Create a thread pool with the number of files
    let pool = worker_pool::create_thread_pool(num_files);

    // Read the CSV files in parallel
    let dfs: Vec<Result<DataFrame, PolarsError>> = pool.install(|| {
        file_paths
            .par_iter()
            .map(|file| process_logs(file.as_str()))
            .collect()
    });
    Ok(dfs.into_iter().filter_map(Result::ok).collect())

}
pub fn print_df_info(df: &DataFrame) {
    println!("DataFrame Info:");
    println!("Shape: {:?}", df.shape());
    println!("Columns: {:?}", df.get_column_names());
    println!("Schema: {:?}", df.schema());
}

pub fn print_df(df: &DataFrame, rows: usize) {
    println!("DataFrame:");
    println!("{:?}", df.head(Some(rows))); // Print the first 10 rows
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

    #[test]
    fn test_multi_files() {
        let file_paths = vec!["preprocessed/Event_traces.csv", "preprocessed/another_file.csv"]; // Replace with your test file paths
        let result = multi_files(file_paths);

        assert!(result.is_ok(), "Failed to process multiple files: {:?}", result.err());

        let dfs = result.unwrap();
        assert!(!dfs.is_empty(), "DataFrames should not be empty");

        // Additional assertions can be added here based on expected DataFrame structure
    }
}