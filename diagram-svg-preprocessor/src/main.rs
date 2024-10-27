mod config;
mod options;
mod svelte;
mod svg;

use anyhow::Result;
use config::Config;
use options::Options;
use svelte::transpiler::Transpiler;
use svg::{css_color::CssColor, modifier::Modifier, parser::Parser, var_color::VarColor};

fn main() -> Result<()> {
    println!("SOAR Diagram SVG Preprocessor CLI");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Author: {}\n", env!("CARGO_PKG_AUTHORS"));

    let mut config = match Config::from_file() {
        Ok(config) => config,
        Err(_) => {
            println!("No configuration file found. Using default configuration.");
            let config = Config::default();

            config
                .save_to_file()
                .expect("Failed to write default configuration to file.");

            config
        }
    };

    loop {
        println!("{}\n{}\n", config, Options);
        print!("Select an option: ");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let option = Options::parse(input.trim());

        match option {
            Some(1) => {
                let raw_svg_text = match config.read_svg_file() {
                    Ok(text) => text,
                    Err(_) => {
                        println!("Failed to read SVG file. Please try again.");
                        continue;
                    }
                };

                let (attributes, original_svg_tag_text) =
                    Parser::parse_attributes(raw_svg_text.as_str())?;
                let mut modifier = Modifier::new(&config, raw_svg_text.as_str());

                let finalized_svg_text = modifier
                    .fix_svg_open_tag(original_svg_tag_text.as_str(), attributes)
                    .map_colors()
                    .accumulate();

                let raw_style_text = match config.read_style_file() {
                    Ok(text) => text,
                    Err(_) => {
                        println!("Failed to read style file. Assuming empty style.");
                        String::new()
                    }
                };

                let raw_svelte_text =
                    Transpiler::to_svelte(finalized_svg_text, "css", raw_style_text.as_str());

                config
                    .output_file(raw_svelte_text.as_str())
                    .expect("Failed to write output file.");

                println!("Successfully processed SVG file.");
            }
            Some(2) => {
                let mut css_color_text = String::new();
                let mut var_color_text = String::new();

                print!("Enter CSS color: ");

                std::io::stdin()
                    .read_line(&mut css_color_text)
                    .expect("Failed to read input.");

                let css_color = match CssColor::new(css_color_text.trim()) {
                    Ok(color) => color,
                    Err(_) => {
                        println!("Invalid CSS color provided. Please try again.");
                        continue;
                    }
                };

                print!("Enter variable color: ");

                std::io::stdin()
                    .read_line(&mut var_color_text)
                    .expect("Failed to read input.");

                let var_color = match VarColor::new(var_color_text.trim()) {
                    Ok(color) => color,
                    Err(_) => {
                        println!("Invalid variable color provided. Please try again.");
                        continue;
                    }
                };

                config.set_color_mapping(css_color, var_color);
            }
            Some(3) => {
                let mut svg_file_path = String::new();

                print!("Enter SVG file path: ");

                std::io::stdin()
                    .read_line(&mut svg_file_path)
                    .expect("Failed to read input.");

                config.set_svg_file_path(svg_file_path.trim());

                println!("SVG file path successfully set.");
            }
            Some(4) => {
                let mut style_file_path = String::new();

                print!("Enter style file path: ");

                std::io::stdin()
                    .read_line(&mut style_file_path)
                    .expect("Failed to read input.");

                config.set_style_file_path(style_file_path.trim());

                println!("Style file path successfully set.");
            }
            Some(5) => break,
            _ => println!("Invalid option selected. Please try again."),
        }
    }

    print!("Saving configuration to file... ");

    config.save_to_file().expect("Failed.");

    println!("Done.");

    Ok(())
}
