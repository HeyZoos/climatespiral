use polars::prelude::*;

fn main() {
    let df = CsvReader::from_path("data/giss.csv")
        .unwrap()
        .with_skip_rows(1)
        .with_null_values(Some(NullValues::AllColumnsSingle("***".to_string())))
        .finish()
        .unwrap();
    dbg!(df);
}
