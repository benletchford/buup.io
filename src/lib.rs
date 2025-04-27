#[doc = include_str!("../README.md")]
use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

pub mod transformers;

// Export the transformer structs for backward compatibility
pub use transformers::{
    AsciiToHex, Base64Decode, Base64Encode, BinaryDecode, BinaryEncode, CamelToSnake, CsvToJson,
    HexDecode, HexEncode, HexToAscii, HtmlDecode, HtmlEncode, JsonFormatter, JsonMinifier,
    JsonToCsv, Md5HashTransformer, Rot13, Sha256HashTransformer, SnakeToCamel, TextReverse,
    UrlDecode, UrlEncode,
};

/// Represents a transformation error
#[derive(Debug)]
pub enum TransformError {
    Base64DecodeError,
    Utf8Error,
    UrlDecodeError,
    UnknownTransformer,
    JsonParseError(String),
    HexDecodeError(String),
    CompressionError(String),
    InvalidArgument(std::borrow::Cow<'static, str>),
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base64DecodeError => write!(f, "Invalid Base64 input"),
            Self::Utf8Error => write!(f, "Invalid UTF-8 in decoded data"),
            Self::UrlDecodeError => write!(f, "Invalid URL-encoded input"),
            Self::UnknownTransformer => write!(f, "Unknown transformer"),
            Self::JsonParseError(details) => write!(f, "JSON parse error: {}", details),
            Self::HexDecodeError(details) => write!(f, "Hex decode error: {}", details),
            Self::CompressionError(details) => {
                write!(f, "Compression/decompression error: {}", details)
            }
            Self::InvalidArgument(details) => write!(f, "Invalid argument: {}", details),
        }
    }
}

impl std::error::Error for TransformError {}

/// Represents the category of a transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformerCategory {
    /// Encoders (e.g., base64encode)
    Encoder,
    /// Decoders (e.g., base64decode)
    Decoder,
    /// Hash functions (e.g., md5hash, sha256hash)
    Crypto,
    /// Formatters (e.g., jsonformatter, jsonminifier)
    Formatter,
    /// Compression transformers (e.g., lzwcompress, lzwdecompress)
    Compression,
    /// Other transformers that don't fit into above categories
    Other,
}

impl std::fmt::Display for TransformerCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoder => write!(f, "encoders"),
            Self::Decoder => write!(f, "decoders"),
            Self::Crypto => write!(f, "crypto"),
            Self::Formatter => write!(f, "formatters"),
            Self::Compression => write!(f, "compression"),
            Self::Other => write!(f, "others"),
        }
    }
}

impl std::str::FromStr for TransformerCategory {
    type Err = TransformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encoders" => Ok(Self::Encoder),
            "decoders" => Ok(Self::Decoder),
            "crypto" => Ok(Self::Crypto),
            "formatters" => Ok(Self::Formatter),
            "compression" => Ok(Self::Compression),
            "others" => Ok(Self::Other),
            _ => Err(TransformError::UnknownTransformer),
        }
    }
}

/// Defines the interface for all transformers
pub trait Transform: Sync + Send {
    /// Get the display name of the transformer
    fn name(&self) -> &'static str;

    /// Get the ID of the transformer
    fn id(&self) -> &'static str;

    /// Get the description of the transformer
    fn description(&self) -> &'static str;

    /// Get the category of the transformer
    fn category(&self) -> TransformerCategory;

    /// Transform the input text
    fn transform(&self, input: &str) -> Result<String, TransformError>;
}

// Static registry of transformers
struct Registry {
    transformers: HashMap<&'static str, &'static dyn Transform>,
}

// Global registry using OnceLock for thread-safe initialization
static REGISTRY: OnceLock<Registry> = OnceLock::new();

// Register built-in transformers
fn register_builtin_transformers() -> Registry {
    let mut registry = Registry {
        transformers: HashMap::new(),
    };

    // Import the new transformer
    use transformers::{
        AsciiToHex, Base64Decode, Base64Encode, BinToDecTransformer, BinToHexTransformer,
        BinaryDecode, BinaryEncode, CamelToSnake, CsvToJson, DecToBinTransformer,
        DecToHexTransformer, HexDecode, HexEncode, HexToAscii, HexToBinTransformer,
        HexToDecTransformer, HtmlDecode, HtmlEncode, JsonFormatter, JsonMinifier, JsonToCsv,
        Md5HashTransformer, Rot13, Sha256HashTransformer, SnakeToCamel, TextReverse, UrlDecode,
        UrlEncode,
    };

    // Register built-in transformers
    registry
        .transformers
        .insert(Base64Encode.id(), &Base64Encode);
    registry
        .transformers
        .insert(Base64Decode.id(), &Base64Decode);
    registry.transformers.insert(UrlEncode.id(), &UrlEncode);
    registry.transformers.insert(UrlDecode.id(), &UrlDecode);
    registry.transformers.insert(TextReverse.id(), &TextReverse);
    registry
        .transformers
        .insert(JsonFormatter.id(), &JsonFormatter);
    registry
        .transformers
        .insert(JsonMinifier.id(), &JsonMinifier);
    registry.transformers.insert(HexEncode.id(), &HexEncode);
    registry.transformers.insert(HexDecode.id(), &HexDecode);
    registry.transformers.insert(HtmlEncode.id(), &HtmlEncode);
    registry.transformers.insert(HtmlDecode.id(), &HtmlDecode);
    registry
        .transformers
        .insert(CamelToSnake.id(), &CamelToSnake);
    registry
        .transformers
        .insert(SnakeToCamel.id(), &SnakeToCamel);
    registry
        .transformers
        .insert(Sha256HashTransformer.id(), &Sha256HashTransformer);
    registry
        .transformers
        .insert(Md5HashTransformer.id(), &Md5HashTransformer);
    registry.transformers.insert(CsvToJson.id(), &CsvToJson);
    registry.transformers.insert(JsonToCsv.id(), &JsonToCsv);
    registry.transformers.insert(Rot13.id(), &Rot13);

    // Register new base conversion transformers
    registry
        .transformers
        .insert(DecToHexTransformer.id(), &DecToHexTransformer);
    registry
        .transformers
        .insert(HexToDecTransformer.id(), &HexToDecTransformer);
    registry
        .transformers
        .insert(DecToBinTransformer.id(), &DecToBinTransformer);
    registry
        .transformers
        .insert(BinToDecTransformer.id(), &BinToDecTransformer);
    registry
        .transformers
        .insert(HexToBinTransformer.id(), &HexToBinTransformer);
    registry
        .transformers
        .insert(BinToHexTransformer.id(), &BinToHexTransformer);

    // Added binary transformers
    registry
        .transformers
        .insert(BinaryEncode.id(), &BinaryEncode);
    registry
        .transformers
        .insert(BinaryDecode.id(), &BinaryDecode);

    registry.transformers.insert(AsciiToHex.id(), &AsciiToHex);
    registry.transformers.insert(HexToAscii.id(), &HexToAscii);

    registry
}

// Initialization helper for the registry
fn get_registry() -> &'static Registry {
    REGISTRY.get_or_init(register_builtin_transformers)
}

/// Returns all available transformers
pub fn all_transformers() -> Vec<&'static dyn Transform> {
    get_registry().transformers.values().copied().collect()
}

/// Find a transformer by its ID
pub fn transformer_from_id(id: &str) -> Result<&'static dyn Transform, TransformError> {
    get_registry()
        .transformers
        .get(id)
        .copied()
        .ok_or(TransformError::UnknownTransformer)
}

/// Get transformer pairs (transformer and its inverse)
pub fn transformer_pairs() -> Vec<(&'static dyn Transform, Option<&'static dyn Transform>)> {
    all_transformers()
        .into_iter()
        .map(|t| (t, inverse_transformer(t)))
        .collect()
}

/// Get the inverse transformer
pub fn inverse_transformer(t: &dyn Transform) -> Option<&'static dyn Transform> {
    match t.id() {
        "base64encode" => transformer_from_id("base64decode").ok(),
        "base64decode" => transformer_from_id("base64encode").ok(),
        "urlencode" => transformer_from_id("urldecode").ok(),
        "urldecode" => transformer_from_id("urlencode").ok(),
        "textreverse" => transformer_from_id("textreverse").ok(), // Self-inverting
        "jsonformatter" => transformer_from_id("jsonminifier").ok(),
        "jsonminifier" => transformer_from_id("jsonformatter").ok(),
        "hexencode" => transformer_from_id("hexdecode").ok(),
        "hexdecode" => transformer_from_id("hexencode").ok(),
        "htmlencode" => transformer_from_id("htmldecode").ok(),
        "htmldecode" => transformer_from_id("htmlencode").ok(),
        "cameltosnake" => transformer_from_id("snaketocamel").ok(),
        "snaketocamel" => transformer_from_id("cameltosnake").ok(),
        "rot13" => transformer_from_id("rot13").ok(),
        // Add quasi-inverses for base conversions (no direct inverse function, but conceptually paired)
        "dec_to_hex" => transformer_from_id("hex_to_dec").ok(),
        "hex_to_dec" => transformer_from_id("dec_to_hex").ok(),
        "dec_to_bin" => transformer_from_id("bin_to_dec").ok(),
        "bin_to_dec" => transformer_from_id("dec_to_bin").ok(),
        "hex_to_bin" => transformer_from_id("bin_to_hex").ok(),
        "bin_to_hex" => transformer_from_id("hex_to_bin").ok(),
        // Added binary transformers
        "binaryencode" => transformer_from_id("binarydecode").ok(),
        "binarydecode" => transformer_from_id("binaryencode").ok(),
        "ascii_to_hex" => transformer_from_id("hex_to_ascii").ok(),
        "hex_to_ascii" => transformer_from_id("ascii_to_hex").ok(),
        _ => None,
    }
}

/// Returns all transformers categorized by their type
pub fn categorized_transformers() -> HashMap<TransformerCategory, Vec<&'static dyn Transform>> {
    let mut categories = HashMap::new();

    // Initialize categories
    categories.insert(TransformerCategory::Encoder, Vec::new());
    categories.insert(TransformerCategory::Decoder, Vec::new());
    categories.insert(TransformerCategory::Formatter, Vec::new());
    categories.insert(TransformerCategory::Crypto, Vec::new());
    categories.insert(TransformerCategory::Compression, Vec::new());
    categories.insert(TransformerCategory::Other, Vec::new());

    // Categorize each transformer using the category method
    for transformer in all_transformers() {
        categories
            .get_mut(&transformer.category())
            .unwrap()
            .push(transformer);
    }

    // Sort each category by transformer ID for consistent ordering
    for transformers in categories.values_mut() {
        transformers.sort_by_key(|t| t.id().to_string());
    }

    categories
}

/// Returns all transformers in a specific category
pub fn get_transformers_by_category(category: TransformerCategory) -> Vec<&'static dyn Transform> {
    categorized_transformers()
        .get(&category)
        .cloned()
        .unwrap_or_default()
}

/// Determines the category of a transformer
pub fn get_transformer_category(transformer: &dyn Transform) -> TransformerCategory {
    transformer.category()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_transformer_from_id() {
        assert_eq!(
            transformer_from_id("base64encode").unwrap().id(),
            "base64encode"
        );
        assert_eq!(
            transformer_from_id("base64decode").unwrap().id(),
            "base64decode"
        );
        assert_eq!(transformer_from_id("urlencode").unwrap().id(), "urlencode");
        assert_eq!(transformer_from_id("urldecode").unwrap().id(), "urldecode");
        assert_eq!(
            transformer_from_id("htmlencode").unwrap().id(),
            "htmlencode"
        );
        assert_eq!(
            transformer_from_id("htmldecode").unwrap().id(),
            "htmldecode"
        );
        assert_eq!(
            transformer_from_id("cameltosnake").unwrap().id(),
            "cameltosnake"
        );
        assert_eq!(
            transformer_from_id("snaketocamel").unwrap().id(),
            "snaketocamel"
        );
        assert_eq!(
            transformer_from_id("sha256hash").unwrap().id(),
            "sha256hash"
        );
        assert_eq!(transformer_from_id("md5hash").unwrap().id(), "md5hash");
        assert_eq!(transformer_from_id("csvtojson").unwrap().id(), "csvtojson");
        assert_eq!(transformer_from_id("jsontocsv").unwrap().id(), "jsontocsv");
        assert!(transformer_from_id("invalid").is_err());
        assert!(transformer_from_id("bin_to_hex").is_ok());
        assert!(transformer_from_id("binaryencode").is_ok());
        assert!(transformer_from_id("binarydecode").is_ok());
        assert!(transformer_from_id("ascii_to_hex").is_ok());
        assert!(transformer_from_id("hex_to_ascii").is_ok());
        assert!(transformer_from_id("nonexistent").is_err());
    }

    #[test]
    fn test_inverse_transformer() {
        // Use a subset of transformers with stable inverse relationships
        let base64encode = transformer_from_id("base64encode").unwrap();
        let base64decode = transformer_from_id("base64decode").unwrap();
        let urlencode = transformer_from_id("urlencode").unwrap();
        let urldecode = transformer_from_id("urldecode").unwrap();
        let sha256hash = transformer_from_id("sha256hash").unwrap();
        let md5hash = transformer_from_id("md5hash").unwrap();

        // Encoder/Decoder pairs
        let inverse_base64encode = inverse_transformer(base64encode);
        assert!(inverse_base64encode.is_some());
        assert_eq!(inverse_base64encode.unwrap().id(), "base64decode");

        let inverse_base64decode = inverse_transformer(base64decode);
        assert!(inverse_base64decode.is_some());
        assert_eq!(inverse_base64decode.unwrap().id(), "base64encode");

        let inverse_urlencode = inverse_transformer(urlencode);
        assert!(inverse_urlencode.is_some());
        assert_eq!(inverse_urlencode.unwrap().id(), "urldecode");

        let inverse_urldecode = inverse_transformer(urldecode);
        assert!(inverse_urldecode.is_some());
        assert_eq!(inverse_urldecode.unwrap().id(), "urlencode");

        // Hash functions have no inverse
        assert!(inverse_transformer(sha256hash).is_none());
        assert!(inverse_transformer(md5hash).is_none());

        assert_eq!(
            inverse_transformer(transformer_from_id("binarydecode").unwrap())
                .unwrap()
                .id(),
            "binaryencode"
        );
        assert_eq!(
            inverse_transformer(transformer_from_id("ascii_to_hex").unwrap())
                .unwrap()
                .id(),
            "hex_to_ascii"
        );
        assert_eq!(
            inverse_transformer(transformer_from_id("hex_to_ascii").unwrap())
                .unwrap()
                .id(),
            "ascii_to_hex"
        );
    }

    #[test]
    fn test_get_transformer_category() {
        assert_eq!(
            get_transformer_category(&Base64Encode),
            TransformerCategory::Encoder
        );
        assert_eq!(
            get_transformer_category(&Base64Decode),
            TransformerCategory::Decoder
        );
        assert_eq!(
            get_transformer_category(&JsonFormatter),
            TransformerCategory::Formatter
        );
        assert_eq!(
            get_transformer_category(&Sha256HashTransformer),
            TransformerCategory::Crypto
        );
        assert_eq!(
            get_transformer_category(&TextReverse),
            TransformerCategory::Other
        );
        assert_eq!(
            get_transformer_category(transformer_from_id("binarydecode").unwrap()),
            TransformerCategory::Decoder
        );
        assert_eq!(
            get_transformer_category(transformer_from_id("ascii_to_hex").unwrap()),
            TransformerCategory::Encoder
        );
        assert_eq!(
            get_transformer_category(transformer_from_id("hex_to_ascii").unwrap()),
            TransformerCategory::Decoder
        );
    }

    #[test]
    fn test_categorized_transformers() {
        let categorized = categorized_transformers();
        assert!(categorized.contains_key(&TransformerCategory::Encoder));
        assert!(categorized.contains_key(&TransformerCategory::Decoder));
        assert!(categorized.contains_key(&TransformerCategory::Crypto));
        assert!(categorized.contains_key(&TransformerCategory::Formatter));
        assert!(categorized.contains_key(&TransformerCategory::Other));

        let encoders = categorized.get(&TransformerCategory::Encoder).unwrap();
        let decoders = categorized.get(&TransformerCategory::Decoder).unwrap();
        let formatters = categorized.get(&TransformerCategory::Formatter).unwrap();
        let crypto = categorized.get(&TransformerCategory::Crypto).unwrap();

        // Check a few specific transformers are in the right category
        assert!(encoders.iter().any(|t| t.id() == "base64encode"));
        assert!(decoders.iter().any(|t| t.id() == "base64decode"));
        assert!(formatters.iter().any(|t| t.id() == "jsonformatter"));
        assert!(crypto.iter().any(|t| t.id() == "sha256hash"));
        assert!(categorized
            .get(&TransformerCategory::Encoder)
            .unwrap()
            .iter()
            .any(|t| t.id() == "ascii_to_hex"));
        assert!(categorized
            .get(&TransformerCategory::Decoder)
            .unwrap()
            .iter()
            .any(|t| t.id() == "hex_to_ascii"));
    }

    #[test]
    fn test_get_transformers_by_category() {
        let encoders = get_transformers_by_category(TransformerCategory::Encoder);
        let decoders = get_transformers_by_category(TransformerCategory::Decoder);

        assert!(!encoders.is_empty());
        assert!(!decoders.is_empty());

        assert!(encoders.iter().any(|t| t.id() == "base64encode"));
        assert!(decoders.iter().any(|t| t.id() == "base64decode"));
    }

    #[test]
    fn test_transformer_category_display() {
        assert_eq!(TransformerCategory::Encoder.to_string(), "encoders");
        assert_eq!(TransformerCategory::Decoder.to_string(), "decoders");
        assert_eq!(TransformerCategory::Crypto.to_string(), "crypto");
        assert_eq!(TransformerCategory::Formatter.to_string(), "formatters");
        assert_eq!(TransformerCategory::Compression.to_string(), "compression");
        assert_eq!(TransformerCategory::Other.to_string(), "others");
    }

    #[test]
    fn test_transformer_category_from_str() {
        assert_eq!(
            TransformerCategory::from_str("encoders").unwrap(),
            TransformerCategory::Encoder
        );
        assert_eq!(
            TransformerCategory::from_str("decoders").unwrap(),
            TransformerCategory::Decoder
        );
        assert_eq!(
            TransformerCategory::from_str("crypto").unwrap(),
            TransformerCategory::Crypto
        );
        assert_eq!(
            TransformerCategory::from_str("formatters").unwrap(),
            TransformerCategory::Formatter
        );
        assert_eq!(
            TransformerCategory::from_str("compression").unwrap(),
            TransformerCategory::Compression
        );
        assert_eq!(
            TransformerCategory::from_str("others").unwrap(),
            TransformerCategory::Other
        );
        assert!(TransformerCategory::from_str("invalid").is_err());
    }
}
