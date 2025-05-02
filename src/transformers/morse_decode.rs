use crate::{Transform, TransformError, TransformerCategory};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Morse decode transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseDecode;

fn build_morse_map() -> HashMap<&'static str, char> {
    let mut map = HashMap::new();
    map.insert(".-", 'A');
    map.insert("-...", 'B');
    map.insert("-.-.", 'C');
    map.insert("-..", 'D');
    map.insert(".", 'E');
    map.insert("..-.", 'F');
    map.insert("--.", 'G');
    map.insert("....", 'H');
    map.insert("..", 'I');
    map.insert(".---", 'J');
    map.insert("-.-", 'K');
    map.insert(".-..", 'L');
    map.insert("--", 'M');
    map.insert("-.", 'N');
    map.insert("---", 'O');
    map.insert(".--.", 'P');
    map.insert("--.-", 'Q');
    map.insert(".-.", 'R');
    map.insert("...", 'S');
    map.insert("-", 'T');
    map.insert("..-", 'U');
    map.insert("...-", 'V');
    map.insert(".--", 'W');
    map.insert("-..-", 'X');
    map.insert("-.--", 'Y');
    map.insert("--..", 'Z');
    map.insert("-----", '0');
    map.insert(".----", '1');
    map.insert("..---", '2');
    map.insert("...--", '3');
    map.insert("....-", '4');
    map.insert(".....", '5');
    map.insert("-....", '6');
    map.insert("--...", '7');
    map.insert("---..", '8');
    map.insert("----.", '9');
    map.insert("/", ' ');
    map
}

static MORSE_MAP: OnceLock<HashMap<&'static str, char>> = OnceLock::new();

fn get_morse_map() -> &'static HashMap<&'static str, char> {
    MORSE_MAP.get_or_init(build_morse_map)
}

/// Default test input for Morse Decode
pub const DEFAULT_TEST_INPUT: &str = ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."; // "HELLO WORLD"

impl Transform for MorseDecode {
    fn name(&self) -> &'static str {
        "Morse Decode"
    }

    fn id(&self) -> &'static str {
        "morsedecode"
    }

    fn description(&self) -> &'static str {
        "Decodes Morse code into text."
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Decoder
    }

    fn default_test_input(&self) -> &'static str {
        ""
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        let morse_map = get_morse_map();
        let mut output = String::new();

        for code in input.trim().split(' ') {
            if code.is_empty() {
                continue;
            }

            match morse_map.get(code) {
                Some(&c) => output.push(c),
                None => {
                    return Err(TransformError::InvalidArgument(
                        format!("Invalid Morse code sequence: {}", code).into(),
                    ))
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morse_decode_basic() {
        let transformer = MorseDecode;
        assert_eq!(
            transformer.transform(DEFAULT_TEST_INPUT).unwrap(),
            "HELLO WORLD"
        );
        assert_eq!(transformer.transform("... --- ...").unwrap(), "SOS");
        assert_eq!(
            transformer
                .transform(".... . .-.. .-.. --- / .-- --- .-. .-.. -..")
                .unwrap(),
            "HELLO WORLD"
        );
        assert_eq!(transformer.transform("  ...   ---   ...  ").unwrap(), "SOS");
        assert_eq!(
            transformer
                .transform(" .... . .-.. .-.. ---  /  .-- --- .-. .-.. -.. ")
                .unwrap(),
            "HELLO WORLD"
        );
    }

    #[test]
    fn test_morse_decode_empty() {
        let transformer = MorseDecode;
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform(" ").unwrap(), "");
    }

    #[test]
    fn test_morse_decode_invalid() {
        let transformer = MorseDecode;
        // Invalid sequence (contains invalid character)
        assert!(transformer.transform(".... x .-..").is_err());
        // Contains valid Morse but not in the map (e.g., punctuation)
        assert!(transformer.transform(".-.-.-").is_err()); // ITU code for period not included
                                                           // Sequence containing valid code mixed with completely invalid code
        assert!(transformer.transform(".--. ---...invalid").is_err());
        // Sequence with only invalid code
        assert!(transformer.transform("invalid code").is_err());
    }
}
