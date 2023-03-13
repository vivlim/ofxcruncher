use std::{path::Path, fs, str::from_utf8};

use anyhow::Context;

use crate::parsing::{self, OfxRoot};

pub fn parse_files(path: &Path) -> anyhow::Result<Vec<(OfxRoot, String)>> {
    let mut results = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("loading {}", path.display());
            let data = fs::read(&path)?;
            // assuming utf8
            let data = from_utf8(&data)?;
            // need to remove lines until the first tag
            let first_tag_index = data.find("<OFX>").context("didn't contain tag")?;
            let parsed = (parsing::parse(&data[first_tag_index..])?, path.to_string_lossy().into_owned());
            results.push(parsed);
        } else {
            println!("skipping {}", path.display());
        }

    }

    Ok(results)
}