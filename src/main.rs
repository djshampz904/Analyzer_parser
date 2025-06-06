mod parser;
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Event Traces Parser",
    version = "1.0",
    author = "Martin Gitau",)
]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Parser)]
enum Commands {
    Print {
        #[clap(long, default_value = "preprocessed/Event_traces.csv")]
        path: String,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
    Schema {
        #[clap(long, default_value = "preprocessed/Event_traces.csv")]
        path: String,
    },
    MultiFiles {
        #[clap(long, value_delimiter = ',', num_args = 1.., value_name = "FILES")]
        paths: Vec<String>,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
    MultiSchema{
         #[clap(long, value_delimiter = ',', num_args = 1.., value_name = "FILES")]
         paths: Vec<String>,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Print { path, rows}) => {
            let df = parser::parser::process_logs(&path).unwrap();
            parser::parser::print_df(&df, rows);
        }
        Some(Commands::Schema {path}) => {
            let df = parser::parser::process_logs(&path).unwrap();
            parser::parser::print_df_info(&df);
        }
        Some(Commands::MultiFiles { paths, rows }) => {
            let collect_paths = paths.iter().map(|s| s.as_str()).collect();
            match parser::parser::multi_files(collect_paths) {
                Ok(dfs) => {
                    for df in dfs {
                        parser::parser::print_df(&df, rows);
                        println!("----------------------------------------------------------------");
                    }
                }
                Err(e) => {
                    eprintln!("Error processing files: {}", e);
                }
            }
        }
        Some(Commands::MultiSchema {paths}) => {
            let collect_paths = paths.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            match parser::parser::multi_files(collect_paths) {
                Ok(dfs) => {
                    for df in dfs {
                        parser::parser::print_df_info(&df);
                        println!("----------------------------------------------------------------");
                    }
                }
                Err(e) => {
                    eprintln!("Error processing files: {}", e);
                }
            }
        }
        _ => {
            println!("No command provided. Use --help for more information.");
        }
    }

}