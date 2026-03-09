use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl PasswordStrength {
    pub fn to_colored_string(&self) -> String {
        match self {
            Self::VeryWeak => "Very Weak".red().to_string(),
            Self::Weak => "Weak".yellow().to_string(),
            Self::Medium => "Medium".blue().to_string(),
            Self::Strong => "Strong".green().to_string(),
            Self::VeryStrong => "Very Strong".bright_green().to_string(),
        }
    }

    fn score(&self) -> u8 {
        match self {
            Self::VeryWeak => 20,
            Self::Weak => 40,
            Self::Medium => 60,
            Self::Strong => 80,
            Self::VeryStrong => 100,
        }
    }

    fn calculate_strength(password: &str) -> Self {
        let length = password.len();
        let mut score = 0;

        match length {
            0..=7 => score = 0,
            8..=12 => score += 10,
            13..=16 => score += 20,
            17..=20 => score += 30,
            _ => score += 40,
        }

        if password.chars().any(|c| c.is_lowercase()) {
            score += 10;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 10;
        }

        if password.chars().any(|c| c.is_numeric()) {
            score += 10;
        }

        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 10;
        }

        if password.chars().filter(|c| c.is_lowercase()).count() > 3 {
            score += 10;
        }

        if password.chars().filter(|c| c.is_uppercase()).count() > 3 {
            score += 10;
        }
        if password.chars().filter(|c| c.is_numeric()).count() > 3 {
            score += 10;
        }
        if password.chars().filter(|c| !c.is_alphanumeric()).count() > 3 {
            score += 10;
        }

        if score < 30 {
            Self::VeryWeak
        } else if score < 50 {
            Self::Weak
        } else if score < 70 {
            Self::Medium
        } else if score < 90 {
            Self::Strong
        } else {
            Self::VeryStrong
        }
    }

    pub fn display_analysis(password: &str) {
        let strength = Self::calculate_strength(password);

        println!("\n{}", "📊 Password Analysis".bright_cyan().bold());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        println!("Length: {} characters", password.len());
        let uppercase_count = password.chars().filter(|c| c.is_uppercase()).count();
        println!("Uppercase: {}", uppercase_count);
        let lowercase_count = password.chars().filter(|c| c.is_lowercase()).count();
        println!("Lowercase: {}", lowercase_count);
        let numeric_count = password.chars().filter(|c| c.is_numeric()).count();
        println!("Numeric: {}", numeric_count);
        let special_count = password.chars().filter(|c| !c.is_alphanumeric()).count();
        println!("Special Characters: {}", special_count);

        println!(
            "\n{}: {} ({}%)\n\n",
            "Strength".bold(),
            strength.to_colored_string(),
            strength.score()
        );

        if strength == Self::Weak || strength == Self::VeryWeak {
            println!(
                "\n{}: Consider using a longer password with a mix of uppercase, lowercase, numbers, and special characters for better security.\n",
                "Tip".yellow().bold()
            );
        }
    }

    pub fn display_analysis_for_passphrase(phrase: &str) {
        let word_count = phrase.split('-').count();
        let strength = if word_count < 3 {
            Self::Weak
        } else if word_count < 5 {
            Self::Medium
        } else if word_count < 7 {
            Self::Strong
        } else {
            Self::VeryStrong
        };

        println!("\n{}", "📊 Passphrase Analysis".bright_cyan().bold());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Words: {}", word_count);
        println!(
            "\n{}: {} ({}%)\n\n",
            "Strength".bold(),
            strength.to_colored_string(),
            strength.score()
        );

        if strength == Self::Weak || strength == Self::VeryWeak {
            println!(
                "\n{}: Consider using more words in your passphrase for better security.\n",
                "Tip".yellow().bold()
            );
        }
    }
}
