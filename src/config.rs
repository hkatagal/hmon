use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeName {
    Default,
    Dracula,
    Nord,
    Gruvbox,
    Cyberpunk,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    #[allow(dead_code)]
    pub background: Color,
    pub text: Color,
}

impl ThemeName {
    pub fn colors(&self) -> ThemeColors {
        match self {
            ThemeName::Default => ThemeColors {
                primary: Color::Cyan,
                secondary: Color::Yellow,
                accent: Color::Green,
                background: Color::Reset,
                text: Color::White,
            },
            ThemeName::Dracula => ThemeColors {
                primary: Color::Rgb(189, 147, 249),   // Purple
                secondary: Color::Rgb(255, 121, 198), // Pink
                accent: Color::Rgb(80, 250, 123),     // Green
                background: Color::Reset,
                text: Color::Rgb(248, 248, 242),
            },
            ThemeName::Nord => ThemeColors {
                primary: Color::Rgb(136, 192, 208),   // Frost Cyan
                secondary: Color::Rgb(129, 161, 193), // Frost Blue
                accent: Color::Rgb(163, 190, 140),    // Green
                background: Color::Reset,
                text: Color::Rgb(236, 239, 244),
            },
            ThemeName::Gruvbox => ThemeColors {
                primary: Color::Rgb(250, 189, 47),  // Yellow
                secondary: Color::Rgb(254, 128, 25), // Orange
                accent: Color::Rgb(184, 187, 38),   // Green
                background: Color::Reset,
                text: Color::Rgb(235, 219, 178),
            },
            ThemeName::Cyberpunk => ThemeColors {
                primary: Color::Rgb(255, 0, 127), // Neon Magenta
                secondary: Color::Rgb(255, 230, 0), // Yellow
                accent: Color::Rgb(0, 255, 204),   // Neon Cyan
                background: Color::Reset,
                text: Color::Rgb(255, 255, 255),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeName,
    pub refresh_interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeName::Default,
            refresh_interval_ms: 500,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_path) = Self::config_file_path() {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(cfg) = toml::from_str::<Config>(&content) {
                        return cfg;
                    }
                }
            }
        }
        Config::default()
    }

    fn config_file_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "hkatagal", "hmon")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors() {
        let theme = ThemeName::Dracula;
        let colors = theme.colors();
        assert_eq!(colors.primary, Color::Rgb(189, 147, 249));
    }

    #[test]
    fn test_config_default() {
        let cfg = Config::default();
        assert_eq!(cfg.theme, ThemeName::Default);
        assert_eq!(cfg.refresh_interval_ms, 500);
    }
}
