use crate::{Transform, TransformError, TransformerCategory};

/// Color Code Converter transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCodeConvert;

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

impl Color {
    fn from_hex(hex: &str) -> Result<Self, TransformError> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 && hex.len() != 8 {
            return Err(TransformError::InvalidArgument(
                "Invalid hex color format".into(),
            ));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| TransformError::InvalidArgument("Invalid hex color".into()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| TransformError::InvalidArgument("Invalid hex color".into()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| TransformError::InvalidArgument("Invalid hex color".into()))?;
        let a = if hex.len() == 8 {
            Some(
                u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| TransformError::InvalidArgument("Invalid hex color".into()))?,
            )
        } else {
            None
        };

        Ok(Color { r, g, b, a })
    }

    fn from_rgb(rgb: &str) -> Result<Self, TransformError> {
        let rgb = rgb.trim_start_matches("rgb(").trim_end_matches(')');
        let parts: Vec<&str> = rgb.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 && parts.len() != 4 {
            return Err(TransformError::InvalidArgument("Invalid RGB format".into()));
        }

        let r = parts[0]
            .parse::<u8>()
            .map_err(|_| TransformError::InvalidArgument("Invalid RGB value".into()))?;
        let g = parts[1]
            .parse::<u8>()
            .map_err(|_| TransformError::InvalidArgument("Invalid RGB value".into()))?;
        let b = parts[2]
            .parse::<u8>()
            .map_err(|_| TransformError::InvalidArgument("Invalid RGB value".into()))?;
        let a = if parts.len() == 4 {
            Some(
                parts[3]
                    .parse::<u8>()
                    .map_err(|_| TransformError::InvalidArgument("Invalid RGB value".into()))?,
            )
        } else {
            None
        };

        Ok(Color { r, g, b, a })
    }

    fn from_hsl(hsl: &str) -> Result<Self, TransformError> {
        let hsl = hsl.trim_start_matches("hsl(").trim_end_matches(')');
        let parts: Vec<&str> = hsl.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 && parts.len() != 4 {
            return Err(TransformError::InvalidArgument("Invalid HSL format".into()));
        }

        let h = parts[0]
            .trim_end_matches("deg")
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid HSL value".into()))?;
        let s = parts[1]
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid HSL value".into()))?
            / 100.0;
        let l = parts[2]
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid HSL value".into()))?
            / 100.0;
        let a = if parts.len() == 4 {
            Some(
                (parts[3]
                    .parse::<f64>()
                    .map_err(|_| TransformError::InvalidArgument("Invalid HSL value".into()))?
                    * 255.0) as u8,
            )
        } else {
            None
        };

        // Convert HSL to RGB
        let (r, g, b) = Self::hsl_to_rgb(h, s, l);
        Ok(Color { r, g, b, a })
    }

    fn from_cmyk(cmyk: &str) -> Result<Self, TransformError> {
        let cmyk = cmyk.trim_start_matches("cmyk(").trim_end_matches(')');
        let parts: Vec<&str> = cmyk.split(',').map(|s| s.trim()).collect();

        if parts.len() != 4 && parts.len() != 5 {
            return Err(TransformError::InvalidArgument(
                "Invalid CMYK format".into(),
            ));
        }

        let c = parts[0]
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid CMYK value".into()))?
            / 100.0;
        let m = parts[1]
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid CMYK value".into()))?
            / 100.0;
        let y = parts[2]
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid CMYK value".into()))?
            / 100.0;
        let k = parts[3]
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| TransformError::InvalidArgument("Invalid CMYK value".into()))?
            / 100.0;
        let a = if parts.len() == 5 {
            Some(
                (parts[4]
                    .parse::<f64>()
                    .map_err(|_| TransformError::InvalidArgument("Invalid CMYK value".into()))?
                    * 255.0) as u8,
            )
        } else {
            None
        };

        // Convert CMYK to RGB
        let r = ((1.0 - c) * (1.0 - k) * 255.0) as u8;
        let g = ((1.0 - m) * (1.0 - k) * 255.0) as u8;
        let b = ((1.0 - y) * (1.0 - k) * 255.0) as u8;

        Ok(Color { r, g, b, a })
    }

    fn to_hex(&self) -> String {
        if let Some(a) = self.a {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, a)
        } else {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }

    fn to_rgb(&self) -> String {
        if let Some(a) = self.a {
            format!("rgb({},{},{},{})", self.r, self.g, self.b, a)
        } else {
            format!("rgb({},{},{})", self.r, self.g, self.b)
        }
    }

    fn to_hsl(&self) -> String {
        let (h, s, l) = Self::rgb_to_hsl(self.r, self.g, self.b);
        if let Some(a) = self.a {
            format!(
                "hsl({:.0}deg,{:.0}%,{:.0}%,{:.2})",
                h,
                s * 100.0,
                l * 100.0,
                a as f64 / 255.0
            )
        } else {
            format!("hsl({:.0}deg,{:.0}%,{:.0}%)", h, s * 100.0, l * 100.0)
        }
    }

    fn to_cmyk(&self) -> String {
        let (c, m, y, k) = Self::rgb_to_cmyk(self.r, self.g, self.b);
        if let Some(a) = self.a {
            format!(
                "cmyk({:.0}%,{:.0}%,{:.0}%,{:.0}%,{:.2})",
                c * 100.0,
                m * 100.0,
                y * 100.0,
                k * 100.0,
                a as f64 / 255.0
            )
        } else {
            format!(
                "cmyk({:.0}%,{:.0}%,{:.0}%,{:.0}%)",
                c * 100.0,
                m * 100.0,
                y * 100.0,
                k * 100.0
            )
        }
    }

    fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match (h / 60.0) as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        let s = if max == min {
            0.0
        } else if l <= 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        let h = if max == min {
            0.0
        } else if max == r {
            60.0 * ((g - b) / (max - min))
        } else if max == g {
            60.0 * (2.0 + (b - r) / (max - min))
        } else {
            60.0 * (4.0 + (r - g) / (max - min))
        };

        (h.rem_euclid(360.0), s, l)
    }

    fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (f64, f64, f64, f64) {
        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;

        let k = 1.0 - r.max(g).max(b);
        if (k - 1.0).abs() < f64::EPSILON {
            // Black
            (0.0, 0.0, 0.0, 1.0)
        } else {
            let c = (1.0 - r - k) / (1.0 - k);
            let m = (1.0 - g - k) / (1.0 - k);
            let y = (1.0 - b - k) / (1.0 - k);
            (c, m, y, k)
        }
    }
}

impl Transform for ColorCodeConvert {
    fn name(&self) -> &'static str {
        "Color Code Converter"
    }

    fn id(&self) -> &'static str {
        "color_code_convert"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn description(&self) -> &'static str {
        "Converts between different color formats (HEX, RGB, HSL, CMYK)"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        let color = if input.starts_with('#') {
            Color::from_hex(input)?
        } else if input.starts_with("rgb(") {
            Color::from_rgb(input)?
        } else if input.starts_with("hsl(") {
            Color::from_hsl(input)?
        } else if input.starts_with("cmyk(") {
            Color::from_cmyk(input)?
        } else {
            return Err(TransformError::InvalidArgument(
                "Unsupported color format".into(),
            ));
        };

        // Convert to all formats
        Ok(format!(
            "HEX: {}\nRGB: {}\nHSL: {}\nCMYK: {}",
            color.to_hex(),
            color.to_rgb(),
            color.to_hsl(),
            color.to_cmyk()
        ))
    }

    fn default_test_input(&self) -> &'static str {
        "#FF0000"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let transformer = ColorCodeConvert;
        let result = transformer.transform("#FF0000").unwrap();
        assert!(result.contains("HEX: #ff0000"));
        assert!(result.contains("RGB: rgb(255,0,0)"));
        assert!(result.contains("HSL: hsl(0deg,100%,50%)"));
        assert!(result.contains("CMYK: cmyk(0%,100%,100%,0%)"));
    }

    #[test]
    fn test_rgb_conversion() {
        let transformer = ColorCodeConvert;
        let result = transformer.transform("rgb(0, 255, 0)").unwrap();
        assert!(result.contains("HEX: #00ff00"));
        assert!(result.contains("RGB: rgb(0,255,0)"));
        assert!(result.contains("HSL: hsl(120deg,100%,50%)"));
        assert!(result.contains("CMYK: cmyk(100%,0%,100%,0%)"));
    }

    #[test]
    fn test_hsl_conversion() {
        let transformer = ColorCodeConvert;
        let result = transformer.transform("hsl(240deg, 100%, 50%)").unwrap();
        assert!(result.contains("HEX: #0000ff"));
        assert!(result.contains("RGB: rgb(0,0,255)"));
        assert!(result.contains("HSL: hsl(240deg,100%,50%)"));
        assert!(result.contains("CMYK: cmyk(100%,100%,0%,0%)"));
    }

    #[test]
    fn test_cmyk_conversion() {
        let transformer = ColorCodeConvert;
        let result = transformer.transform("cmyk(0%, 0%, 0%, 100%)").unwrap();
        println!("CMYK conversion result: {}", result);
        assert!(result.contains("HEX: #000000"));
        assert!(result.contains("RGB: rgb(0,0,0)"));
        assert!(result.contains("HSL: hsl(0deg,0%,0%)"));
        assert!(result.contains("CMYK: cmyk(0%,0%,0%,100%)"));
    }

    #[test]
    fn test_invalid_input() {
        let transformer = ColorCodeConvert;
        assert!(transformer.transform("invalid").is_err());
        assert!(transformer.transform("#GG0000").is_err());
        assert!(transformer.transform("rgb(300,0,0)").is_err());
    }
}
