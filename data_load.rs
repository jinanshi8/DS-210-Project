use polars::prelude::*;


pub fn load_data(path: &str) -> DataFrame {
    let df = CsvReader::from_path(path)
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    df
}

