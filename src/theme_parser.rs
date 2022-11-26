use std::{fs, path::PathBuf};

use gtk::{gio::Settings, prelude::SettingsExt};
use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ColorScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    accent_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accent_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accent_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    destructive_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destructive_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destructive_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    success_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    warning_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    window_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    view_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headerbar_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headerbar_fg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headerbar_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headerbar_backdrop_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headerbar_shade_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    card_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_fg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_shade_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dialog_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialog_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    popover_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    popover_fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shade_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scrollbar_outline_color: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Colors {
    light: ColorScheme,
    dark: ColorScheme,
    global: ColorScheme,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeConfig {
    flags: (String, String),
    main: Colors,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            flags: (String::from("light"), String::from("dark")),
            main: Colors::default(),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            light: ColorScheme::default(),
            dark: ColorScheme::default(),
            global: ColorScheme {
                accent_color: None,
                accent_bg_color: Some(HexColor::random_rgb().to_string()),
                accent_fg_color: Some(HexColor::random_rgb().to_string()),
                destructive_color: None,
                destructive_bg_color: None,
                destructive_fg_color: Some(HexColor::random_rgb().to_string()),
                success_color: None,
                success_bg_color: None,
                success_fg_color: Some(HexColor::random_rgb().to_string()),
                warning_color: None,
                warning_bg_color: None,
                warning_fg_color: Some(HexColor::random_rgb().to_string()),
                error_color: None,
                error_bg_color: None,
                error_fg_color: Some(HexColor::random_rgb().to_string()),
                window_bg_color: None,
                window_fg_color: None,
                view_bg_color: None,
                view_fg_color: None,
                headerbar_bg_color: None,
                headerbar_fg_color: None,
                headerbar_border_color: None,
                headerbar_backdrop_color: None,
                headerbar_shade_color: None,
                card_bg_color: None,
                card_fg_color: None,
                card_shade_color: None,
                dialog_bg_color: None,
                dialog_fg_color: None,
                popover_bg_color: None,
                popover_fg_color: None,
                shade_color: None,
                scrollbar_outline_color: None,
            },
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            accent_color: Some(HexColor::random_rgb().to_string()),
            accent_bg_color: None,
            accent_fg_color: None,
            destructive_color: Some(HexColor::random_rgb().to_string()),
            destructive_bg_color: Some(HexColor::random_rgb().to_string()),
            destructive_fg_color: None,
            success_color: Some(HexColor::random_rgb().to_string()),
            success_bg_color: Some(HexColor::random_rgb().to_string()),
            success_fg_color: None,
            warning_color: Some(HexColor::random_rgb().to_string()),
            warning_bg_color: Some(HexColor::random_rgb().to_string()),
            warning_fg_color: None,
            error_color: Some(HexColor::random_rgb().to_string()),
            error_bg_color: Some(HexColor::random_rgb().to_string()),
            error_fg_color: None,
            window_bg_color: Some(HexColor::random_rgb().to_string()),
            window_fg_color: Some(HexColor::random_rgb().to_string()),
            view_bg_color: Some(HexColor::random_rgb().to_string()),
            view_fg_color: Some(HexColor::random_rgb().to_string()),
            headerbar_bg_color: Some(HexColor::random_rgb().to_string()),
            headerbar_fg_color: Some(HexColor::random_rgb().to_string()),
            headerbar_border_color: Some(HexColor::random_rgb().to_string()),
            headerbar_backdrop_color: Some(HexColor::random_rgb().to_string()),
            headerbar_shade_color: Some(HexColor::random_rgb().to_string()),
            card_bg_color: Some(HexColor::random_rgb().to_string()),
            card_fg_color: Some(HexColor::random_rgb().to_string()),
            card_shade_color: Some(HexColor::random_rgb().to_string()),
            dialog_bg_color: Some(HexColor::random_rgb().to_string()),
            dialog_fg_color: Some(HexColor::random_rgb().to_string()),
            popover_bg_color: Some(HexColor::random_rgb().to_string()),
            popover_fg_color: Some(HexColor::random_rgb().to_string()),
            shade_color: Some(HexColor::random_rgb().to_string()),
            scrollbar_outline_color: Some(HexColor::random_rgb().to_string()),
        }
    }
}

/// Saves this creation
pub fn serialise_hell() {
    let theme = ThemeConfig::default();

    let mut theme_loc: PathBuf = PathBuf::new();
    theme_loc.push(dirs::home_dir().expect("why the fuck is there no home folder"));
    theme_loc.push(".rthemes");

    fs::create_dir_all(&theme_loc).expect("Couldn't create theme directory");

    theme_loc.push("rtheme_hell.yml");
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(theme_loc)
        .expect("Couldn't open file");

    serde_yaml::to_writer(file, &theme).expect("Couldn't write theme");
}

/// Save theme
pub fn save_theme_settings() {
    let settings = Settings::new("io.risi.rtheme");

    settings.set_string("theme-name", "rtheme_hell").expect("Couldn't set theme");
}