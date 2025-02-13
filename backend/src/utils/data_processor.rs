// utils/data_processor.rs
use polars::prelude::*;

/// Processes data from a CSV file into a DataFrame.
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

/// Additional component 3: Function to sort the DataFrame by a specific column
pub fn sort_data(df: &DataFrame, column: &str, ascending: bool) -> DataFrame {
    df.sort(vec![column], vec![ascending]).unwrap()
}

/// Additional component 4: Function to add a new column with computed values
pub fn add_computed_column(df: &mut DataFrame, new_column: &str, computation: fn(&Series) -> Series) {
    let computed_values = computation(df.column("value").unwrap());
    df.with_column(computed_values.alias(new_column)).unwrap();
}

/// Additional component 5: Function to export DataFrame to a CSV file
pub fn export_to_csv(df: &DataFrame, file_path: &str) {
    let mut file = std::fs::File::create(file_path).unwrap();
    CsvWriter::new(&mut file).finish(df).unwrap();
}

/// Example usage of the data processing utilities
fn main() {
    let df = process_data("data.csv");
    println!("Original DataFrame:\n{}", df);

    let filtered_df = filter_data(&df, "category", "A");
    println!("Filtered DataFrame:\n{}", filtered_df);

    let mean_value = calculate_summary_statistics(&df, "value");
    println!("Mean of 'value' column: {}", mean_value);

    let sorted_df = sort_data(&df, "value", false);
    println!("Sorted DataFrame:\n{}", sorted_df);

    let mut df_with_computed_column = df.clone();
    add_computed_column(&mut df_with_computed_column, "value_squared", |s| s * s);
    println!("DataFrame with computed column:\n{}", df_with_computed_column);

    export_to_csv(&df_with_computed_column, "output.csv");
    println!("DataFrame exported to 'output.csv'");
}
