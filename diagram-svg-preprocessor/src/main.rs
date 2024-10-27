mod config;
mod svelte;
mod svg;

use anyhow::Result;
use config::Config;

fn main() -> Result<()> {
    println!("SOAR Diagram SVG Preprocessor CLI");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Author: {}\n", env!("CARGO_PKG_AUTHORS"));

    let config = match Config::from_file() {
        Ok(config) => config,
        Err(_) => {
            println!("No configuration file found. Using default configuration.");
            let config = Config::default();

            config
                .to_file()
                .expect("Failed to write default configuration to file.");

            config
        }
    };

    println!("{}\n", config);

    let raw_style_text = match config.read_style_file() {
        Ok(text) => text,
        Err(_) => {
            println!("Failed to read style file. Assuming empty style.");
            String::new()
        }
    };

    Ok(())
}
