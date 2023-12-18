use std::sync::RwLock;

use hashbrown::HashMap;
use once_cell::sync::Lazy;
use rusttype::Font;

use crate::prelude::SuperFont;

static FONT_DB: Lazy<RwLock<HashMap<String, Font<'static>>>> =
    Lazy::new(|| RwLock::new(HashMap::default()));

#[cfg(feature = "emoji")]
static DEFAULT_EMOJI_OPTIONS: Lazy<RwLock<crate::prelude::EmojiOptions>> =
    Lazy::new(|| RwLock::new(crate::prelude::EmojiOptions::default()));

pub struct FontDB;

impl FontDB {
    pub fn inner() -> &'static RwLock<HashMap<String, Font<'static>>> {
        &FONT_DB
    }

    pub fn insert<S: Into<String>>(name: S, font: Font<'static>) -> Result<(), &'static str> {
        match FONT_DB.write() {
            Ok(mut db) => {
                db.insert(name.into(), font);
                Ok(())
            }
            Err(_) => return Err("Failed to write to font database"),
        }
    }

    pub fn load_font_data<S: Into<String>>(name: S, data: Vec<u8>) -> Result<(), &'static str> {
        match Font::try_from_vec(data) {
            Some(font) => Self::insert(name, font),
            None => Err("Failed to load font"),
        }
    }

    pub fn load_from_path<S: Into<String>, P: AsRef<std::path::Path>>(
        name: S,
        path: P,
    ) -> Result<(), String> {
        match std::fs::read(path) {
            Ok(data) => match Self::load_font_data(name, data) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(format!("Failed to read font file: {}", e)),
        }
    }

    pub fn load_from_dir<P: AsRef<std::path::Path>>(path: P) {
        let dir = match std::fs::read_dir(path) {
            Ok(dir) => dir,
            Err(e) => {
                log::warn!("Failed to read font directory: {}", e);
                return;
            }
        };

        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    match path.extension().and_then(|ext| ext.to_str()) {
                        Some("ttf") | Some("otf") | Some("TTF") | Some("OTF") => {
                            match path.file_stem().and_then(|stem| stem.to_str()) {
                                Some(name) => match Self::load_from_path(name, &path) {
                                    Ok(_) => log::info!("Loaded font: {}", name),
                                    Err(e) => log::warn!("Failed to load font: {}", e),
                                },
                                None => log::warn!("Failed to load font: {}", path.display()),
                            }
                        }
                        _ => (),
                    }
                } else if path.is_dir() {
                    Self::load_from_dir(path);
                }
            }
        }
    }

    pub fn load_system_fonts() {
        #[cfg(target_os = "windows")]
        {
            Self::load_from_dir("C:\\Windows\\Fonts\\");
            if let Ok(ref home) = std::env::var("USERPROFILE") {
                let home_path = std::path::Path::new(home);
                Self::load_from_dir(home_path.join("AppData\\Local\\Microsoft\\Windows\\Fonts"));
                Self::load_from_dir(home_path.join("AppData\\Roaming\\Microsoft\\Windows\\Fonts"));
            }
        }

        #[cfg(target_os = "macos")]
        {
            Self::load_from_dir("/Library/Fonts");
            Self::load_from_dir("/System/Library/Fonts");
            Self::load_from_dir("/System/Library/AssetsV2/com_apple_MobileAsset_Font6");
            Self::load_from_dir("/Network/Library/Fonts");

            if let Ok(ref home) = std::env::var("HOME") {
                let home_path = std::path::Path::new(home);
                Self::load_from_dir(home_path.join("Library/Fonts"));
            }
        }

        #[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
        {
            Self::load_from_dir("/usr/share/fonts");
            Self::load_from_dir("/usr/local/share/fonts");

            if let Ok(ref home) = std::env::var("HOME") {
                let home_path = std::path::Path::new(home);
                Self::load_from_dir(home_path.join(".fonts"));
                Self::load_from_dir(home_path.join(".local/share/fonts"));
            }
        }
    }

    pub fn get(name: &str) -> Option<Font<'static>> {
        match FONT_DB.read() {
            Ok(db) => match db.get(name) {
                Some(font) => Some(font.clone()),
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn superfont<'a>(font_names: &[&str]) -> Option<SuperFont<'a>> {
        match FONT_DB.read() {
            Ok(db) => {
                let mut fonts = font_names.into_iter().filter_map(|name| db.get(*name));

                #[cfg(not(feature = "emoji"))]
                return Some(SuperFont::new(
                    fonts.next()?.clone(),
                    fonts.map(|f| f.clone()).collect::<Vec<Font<'static>>>(),
                ));

                #[cfg(feature = "emoji")]
                return Some(SuperFont::with_emoji_options(
                    fonts.next()?.clone(),
                    fonts.map(|f| f.clone()).collect::<Vec<Font<'static>>>(),
                    DEFAULT_EMOJI_OPTIONS
                        .read()
                        .expect("Failed to read emoji options")
                        .clone(),
                ));
            }
            Err(_) => None,
        }
    }

    pub fn query<'a>(query: &str) -> Option<SuperFont<'a>> {
        Self::superfont(&query.split_whitespace().collect::<Vec<&str>>())
    }

    pub fn remove(name: &str) -> Result<(), &'static str> {
        match FONT_DB.write() {
            Ok(mut db) => {
                db.remove(name);
                Ok(())
            }
            Err(_) => Err("Failed to write to font database"),
        }
    }

    #[cfg(feature = "emoji")]
    pub fn superfont_with_emoji<'a>(
        font_names: &[&str],
        emoji_options: crate::prelude::EmojiOptions,
    ) -> Option<SuperFont<'a>> {
        match FONT_DB.read() {
            Ok(db) => {
                let mut fonts = font_names.into_iter().filter_map(|name| db.get(*name));

                Some(SuperFont::with_emoji_options(
                    fonts.next()?.clone(),
                    fonts.map(|f| f.clone()).collect::<Vec<Font<'static>>>(),
                    emoji_options,
                ))
            }
            Err(_) => None,
        }
    }

    #[cfg(feature = "emoji")]
    pub fn query_with_emoji<'a>(
        query: &str,
        emoji_options: crate::prelude::EmojiOptions,
    ) -> Option<SuperFont<'a>> {
        Self::superfont_with_emoji(
            &query.split_whitespace().collect::<Vec<&str>>(),
            emoji_options,
        )
    }

    #[cfg(feature = "emoji")]
    pub fn set_default_emoji_options(emoji_options: crate::prelude::EmojiOptions) {
        match DEFAULT_EMOJI_OPTIONS.write() {
            Ok(mut options) => *options = emoji_options,
            Err(_) => {}
        }
    }
}
