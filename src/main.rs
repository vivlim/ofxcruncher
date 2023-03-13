use std::path::Path;

mod parsing;
mod collector;


fn main() -> anyhow::Result<()> {
    let result = collector::parse_files(Path::new("inputs"))?;
    println!("{:?}", result);
    Ok(())
}
