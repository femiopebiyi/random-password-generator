use std::char;

use rand::{RngExt, rng, seq::SliceRandom};

pub struct PasswordGenerator {
    length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_special: bool,
    exclude_ambiguous: bool,
}

impl PasswordGenerator {
    pub fn new(
        length: usize,
        include_uppercase: bool,
        include_lowercase: bool,
        include_numbers: bool,
        include_special: bool,
        exclude_ambiguous: bool,
    ) -> Self {
        Self {
            length,
            include_uppercase,
            include_lowercase,
            include_numbers,
            include_special,
            exclude_ambiguous,
        }
    }

    pub fn generate(&self) -> Result<String, String> {
        if self.length < 4 || self.length > 50 {
            return Err("Password characters out of range (0-50)".to_string());
        }
        if !self.include_lowercase
            && !self.include_uppercase
            && !self.include_numbers
            && self.exclude_ambiguous
            && !self.include_special
        {
            return Err("At least one character type must be included".to_string());
        }

        let mut charset = String::new();
        if self.include_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.include_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.include_numbers {
            charset.push_str("0123456789");
        }
        if self.include_special {
            charset.push_str("!@#$%^&*()-+");
        }

        if self.exclude_ambiguous {
            charset.retain(|c| c != '0' && c != 'O' && c != 'l' && c != '1' && c != 'I');
        }

        if charset.is_empty() {
            return Err(
                "Password container is empty, At least one character type must be included"
                    .to_string(),
            );
        }

        let shuffled = shuffle_string(charset);

        let mut rng = rand::rng();
        let password = (0..self.length)
            .map(|_| {
                let idx = rng.random_range(0..shuffled.len());
                shuffled.chars().nth(idx).unwrap()
            })
            .collect::<String>();

        Ok(password)
    }

    pub fn _get_charset(&self) -> String {
        if !self.include_lowercase
            && !self.include_uppercase
            && !self.include_numbers
            && !self.exclude_ambiguous
            && !self.include_special
        {
            return String::new();
        }
        let mut charset = String::new();
        if self.include_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.include_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.include_numbers {
            charset.push_str("0123456789");
        }
        if self.include_special {
            charset.push_str("!@#$%^&*()-+");
        }
        if self.exclude_ambiguous {
            charset = charset
                .replace("0", "")
                .replace("O", "")
                .replace("l", "")
                .replace("1", "")
                .replace("I", "");
        }
        charset
    }

    pub fn _is_ambiguous(c: char) -> bool {
        c == '0' || c == 'O' || c == 'l' || c == '1' || c == 'I'
    }
}

fn shuffle_string(s: String) -> String {
    // Convert to Vec<char> (handles Unicode properly)
    let mut chars: Vec<char> = s.chars().collect();

    // Shuffle the vector
    chars.shuffle(&mut rng());

    // Convert back to String
    chars.into_iter().collect()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn check_generated_length() {
        let password_gen = PasswordGenerator::new(10, false, false, false, false, false);

        assert_eq!(password_gen.length, 10);
    }

    #[test]
    #[should_panic]
    fn check_if_not_when_at_least_one_character_type_is_not_included() {
        let password_gen = PasswordGenerator::new(13, false, false, false, false, false);
        let password = password_gen.generate();

        match password {
            Ok(extracted_password) => assert_eq!(extracted_password.len(), 13),
            Err(error) => {
                println!("{}", error);
                panic!();
            }
        }
    }

    #[test]
    fn check_if_password_contains_ambiguous_characters() {
        let password_gen = PasswordGenerator::new(30, true, true, true, false, false);
        let password = password_gen.generate();
        println!("{}", password.clone().unwrap());

        let check = password
            .unwrap()
            .chars()
            .any(|c| PasswordGenerator::_is_ambiguous(c));

        assert!(check);
    }

    #[test]
    fn return_error_when_length_is_out_of_range() {
        let password_gen_1 = PasswordGenerator::new(51, true, true, true, false, false);
        let password_gen_2 = PasswordGenerator::new(3, true, true, true, false, false);

        assert!(password_gen_1.generate().is_err() && password_gen_2.generate().is_err());
    }
}
