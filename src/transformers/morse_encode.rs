use crate::{Transform, TransformError, TransformerCategory};

/// Morse encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseEncode;

impl Transform for MorseEncode {
    fn name(&self) -> &'static str {
        "Morse Encode"
    }

    fn id(&self) -> &'static str {
        "morseencode"
    }

    fn description(&self) -> &'static str {
        "Encode text to Morse code"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Encoder
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut output = String::new();
        for (i, c) in input.to_uppercase().chars().enumerate() {
            if i > 0 {
                output.push(' '); // Word separator (space between morse codes)
            }

            let code = match c {
                'A' => ".-",
                'B' => "-...",
                'C' => "-.-.",
                'D' => "-..",
                'E' => ".",
                'F' => "..-.",
                'G' => "--.",
                'H' => "....",
                'I' => "..",
                'J' => ".---",
                'K' => "-.-",
                'L' => ".-..",
                'M' => "--",
                'N' => "-.",
                'O' => "---",
                'P' => ".--.",
                'Q' => "--.-",
                'R' => ".-.",
                'S' => "...",
                'T' => "-",
                'U' => "..-",
                'V' => "...-",
                'W' => ".--",
                'X' => "-..-",
                'Y' => "-.--",
                'Z' => "--..",
                '0' => "-----",
                '1' => ".----",
                '2' => "..---",
                '3' => "...--",
                '4' => "....-",
                '5' => ".....",
                '6' => "-....",
                '7' => "--...",
                '8' => "---..",
                '9' => "----.",
                ' ' => "/", // Space is represented by a forward slash
                _ => {
                    return Err(TransformError::InvalidArgument(
                        format!("Cannot encode '{}' to Morse code", c).into(),
                    ))
                }
            };
            output.push_str(code);
        }

        Ok(output)
    }

    fn default_test_input(&self) -> &'static str {
        "Hello World"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morse_encode_basic() {
        let transformer = MorseEncode;
        assert_eq!(transformer.transform("SOS").unwrap(), "... --- ...");
        assert_eq!(
            transformer
                .transform(transformer.default_test_input())
                .unwrap(),
            ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
        );
    }

    #[test]
    fn test_morse_encode_empty() {
        let transformer = MorseEncode;
        assert_eq!(transformer.transform("").unwrap(), "");
    }

    #[test]
    fn test_morse_encode_invalid() {
        let transformer = MorseEncode;
        assert!(transformer.transform("@").is_err());
    }
}
