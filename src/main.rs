use polars::prelude::*;

fn main() {
    let df = CsvReader::from_path("data/giss.csv")
        .unwrap()
        .with_skip_rows(1)
        .finish()
        .unwrap();
    dbg!(df);
}
