use polars::prelude::*;

fn main() {
    let df = CsvReader::from_path("data/giss.csv")
        .unwrap()
        .finish()
        .unwrap();
    dbg!(df);
}
