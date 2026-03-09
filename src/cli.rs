use clap::Parser;

use crate::generator::PasswordType;

/// A secure password generator
#[derive(Parser, Debug)]
#[command(name = "passgen")]
#[command(about = "Generate secure random passwords", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Length of the password (default: 16)
    #[arg(short = 'L', long, default_value_t = 16)]
    pub length: usize,

    /// Include uppercase letters (A-Z)
    #[arg(short = 'u', long, default_value_t = true, action = clap::ArgAction::Set)]
    pub include_uppercase: bool,

    /// Include lowercase letters (a-z)
    #[arg(short = 'l', long, default_value_t = true, action = clap::ArgAction::Set)]
    pub include_lowercase: bool,

    /// Include numbers (0-9)
    #[arg(short = 'n', long, default_value_t = true, action = clap::ArgAction::Set)]
    pub include_numbers: bool,

    /// Include special characters (!@#$%^&*...)
    #[arg(short = 's', long, default_value_t = false, action = clap::ArgAction::Set)]
    pub include_special: bool,

    /// Number of passwords to generate
    #[arg(short = 'c', long, default_value_t = 1)]
    pub count: usize,

    /// Show password strength analysis
    #[arg(long, short = 'a', default_value_t = false)]
    pub show_strength: bool,

    /// Exclude ambiguous characters (0, O, l, 1, I)
    #[arg(long, default_value_t = false)]
    pub exclude_ambiguous: bool,

    /// Type of password to generate
    #[arg(short = 't', long, default_value_t = PasswordType::PassWord, value_enum)]
    pub password_type: PasswordType,
}
