// utils/data_processor.rs
use polars::prelude::*;

pub fn process_data(data: &str) -> DataFrame {
    let mut df = CsvReader::from_path(data).unwrap().finish().unwrap();
    df
}
