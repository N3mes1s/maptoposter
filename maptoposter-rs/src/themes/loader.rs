use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde_json::Value;

/// Load all themes from the themes directory
pub fn load_themes(themes_dir: &Path) -> HashMap<String, Value> {
    let mut themes = HashMap::new();

    let entries = match fs::read_dir(themes_dir) {
        Ok(entries) => entries,
        Err(e) => {
            tracing::warn!("Could not read themes directory {:?}: {}", themes_dir, e);
            return themes;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                match load_theme_file(&path) {
                    Ok(theme) => {
                        themes.insert(name.to_string(), theme);
                    }
                    Err(e) => {
                        tracing::warn!("Could not load theme {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    themes
}

/// Load a specific theme by name
pub fn load_theme(themes_dir: &Path, name: &str) -> Option<Value> {
    let path = themes_dir.join(format!("{}.json", name));
    load_theme_file(&path).ok()
}

/// Load a theme from a file path
fn load_theme_file(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let theme: Value = serde_json::from_str(&content)?;
    Ok(theme)
}

/// Get a color from a theme, with a fallback default
pub fn get_theme_color(theme: &Value, key: &str, default: &str) -> String {
    theme
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

/// Parse a hex color string to RGB components
pub fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

/// Parse a hex color string to RGBA with alpha
pub fn parse_hex_color_rgba(hex: &str, alpha: u8) -> Option<(u8, u8, u8, u8)> {
    let (r, g, b) = parse_hex_color(hex)?;
    Some((r, g, b, alpha))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_hex_color("#FFFFFF"), Some((255, 255, 255)));
        assert_eq!(parse_hex_color("#000000"), Some((0, 0, 0)));
        assert_eq!(parse_hex_color("#FF0000"), Some((255, 0, 0)));
        assert_eq!(parse_hex_color("FFFFFF"), Some((255, 255, 255)));
        assert_eq!(parse_hex_color("#FFF"), None); // Invalid length
    }
}
