use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::create("foo.txt")?;
    let metadata = f.metadata()?;

    assert_eq!(false, metadata.permissions().readonly());
    Ok(())
}
