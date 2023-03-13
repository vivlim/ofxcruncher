use std::path::Path;

use rust_decimal::Decimal;
use serde::Deserialize;

mod sample;
mod parsing;
mod collector;


fn main() -> anyhow::Result<()> {
    let result = collector::parse_files(Path::new("inputs"))?;
    println!("{:?}", result);
    Ok(())
}
