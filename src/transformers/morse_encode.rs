use crate::{Transform, TransformError, TransformerCategory};

/// Morse encode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseEncode;

/// Default test input for Morse Encode
pub const DEFAULT_TEST_INPUT: &str = "Hello World";

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
                output.push(' ');
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
                ' ' => "/",
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morse_encode_basic() {
        let transformer = MorseEncode;
        assert_eq!(transformer.transform("SOS").unwrap(), "... --- ...");
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
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
