// utils/data_processor.rs
use polars::prelude::*;

pub fn process_data(data: &str) -> DataFrame {
    let mut df = CsvReader::from_path(data).unwrap().finish().unwrap();
    df
}

/// Additional component 1: Function to filter rows based on a condition
pub fn filter_data(df: &DataFrame, column: &str, value: &str) -> DataFrame {
    df.filter(&df.column(column).unwrap().equal(value).unwrap()).unwrap()
}

/// Additional component 2: Function to calculate summary statistics for a column
pub fn calculate_summary_statistics(df: &DataFrame, column: &str) -> Series {
    df.column(column).unwrap().mean().unwrap()
}

/// Example usage of the data processing utilities
fn main() {
    let df = process_data("data.csv");
    println!("Original DataFrame:\n{}", df);

    let filtered_df = filter_data(&df, "category", "A");
    println!("Filtered DataFrame:\n{}", filtered_df);

    let mean_value = calculate_summary_statistics(&df, "value");
    println!("Mean of 'value' column: {}", mean_value);
}
