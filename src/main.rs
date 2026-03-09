use crate::{cli::Cli, generator::PasswordGenerator};
use clap::Parser;
use colored::Colorize;
use strength::PasswordStrength;

mod cli;
mod generator;
mod strength;

fn main() {
    let args = Cli::parse();

    let generator = PasswordGenerator::new(
        args.length,
        args.include_uppercase,
        args.include_lowercase,
        args.include_numbers,
        args.include_special,
        args.exclude_ambiguous,
    );

    for i in 0..args.count {
        match generator.generate() {
            Ok(password) => {
                if args.count > 1 {
                    println!("🔐 Password {}: {}", i + 1, password.bright_green().bold());
                } else {
                    println!("🔐 Generated Password: {}", password.bright_green().bold());
                }

                if args.show_strength {
                    PasswordStrength::display_analysis(&password);
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e.red());
                std::process::exit(0);
            }
        }
    }
}
