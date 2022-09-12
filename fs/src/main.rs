use std::{fs, path::Path};

fn main() -> std::io::Result<()> {
    let con = fs::read_dir(Path::new("."))?;

    Ok(())
}
