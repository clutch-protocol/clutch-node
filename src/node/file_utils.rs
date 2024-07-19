use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn write_to_file(content: &str, file_name: &str) -> io::Result<()> {
    let output_dir = "output";
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    let address = format!("{}/{}.json", output_dir, file_name);
    let path = Path::new(&address);

    let mut file = File::create(&path)?;

    file.write_all(content.as_bytes())?;

    println!(
        "Content has been successfully saved to '{}'.",
        path.display()
    );
    Ok(())
}