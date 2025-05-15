use rayon::prelude::*;
use rayon::{max_num_threads, ThreadPoolBuilder};

pub fn calculate_optimal_threads(file_count: usize) -> usize {
    // Calculate the optimal number of threads based on the number of files
    let max_threads = num_cpus::get();
    println!("Available threads: {}", max_threads);

    let optimal_threads = if file_count > max_threads {
        max_threads
    } else {
        file_count
    };
    optimal_threads

}
pub fn create_thread_pool(num_threads: usize) -> rayon::ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to create thread pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thread_pool() {
        let num_threads = 4;
        let thread_pool = create_thread_pool(num_threads);

        assert_eq!(thread_pool.current_num_threads(), num_threads, "Thread pool should have {} threads", num_threads);
    }
    #[test]
    fn test_calculate_optimal_threads() {
        let file_count = 10;
        let optimal_threads = calculate_optimal_threads(file_count);

        assert!(optimal_threads > 0, "Optimal threads should be greater than 0");
        assert!(optimal_threads <= num_cpus::get(), "Optimal threads should not exceed available CPU cores");
    }

}