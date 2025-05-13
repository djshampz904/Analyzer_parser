mod parser;

fn main() {
    let file_path = "preprocessed/Event_traces.csv"; // Replace with your file path

    // Process the logs and get the DataFrame
    let result = parser::parser::process_logs(file_path);
    match result {
        Ok(df) => {
            // Print DataFrame information
            parser::parser::print_df_info(&df);
        }
        Err(e) => {
            eprintln!("Error processing logs: {:?}", e);
        }
    }
}
