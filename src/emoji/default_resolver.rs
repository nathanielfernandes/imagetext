use hashbrown::HashMap;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::sync::RwLock;

use super::{
    source::EmojiPath,
    source::{EmojiResolver, ResolvedEmoji, UnresolvedEmoji},
};

static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(|| reqwest::blocking::Client::new());
static LOCAL_CACHE: Lazy<RwLock<HashMap<String, Option<image::RgbaImage>>>> =
    Lazy::new(|| RwLock::new(HashMap::default()));
static EXTERNAL_CACHE: Lazy<Cache<String, Option<image::RgbaImage>>> = Lazy::new(|| {
    Cache::builder()
        .time_to_idle(std::time::Duration::from_secs(60 * 10))
        .build()
});

pub struct DefaultEmojiResolver<const RESOLVE_DISCORD: bool>;

fn resize(image: &image::RgbaImage, size: u32) -> image::RgbaImage {
    let (mut width, mut height) = image.dimensions();

    if width > height {
        height = (height as f32 / width as f32 * size as f32) as u32;
        width = size;
    } else {
        width = (width as f32 / height as f32 * size as f32) as u32;
        height = size;
    }

    image::imageops::resize(image, width, height, image::imageops::FilterType::Lanczos3)
}

impl<const RESOLVE_DISCORD: bool> DefaultEmojiResolver<RESOLVE_DISCORD> {
    fn resolve_emoji(emoji: &EmojiPath, size: u32) -> Option<image::RgbaImage> {
        match emoji {
            EmojiPath::Local(path) => {
                match match LOCAL_CACHE.read() {
                    Ok(cache) => match cache.get(path) {
                        Some(found) => match found {
                            Some(image) => Some(Some(resize(image, size))),
                            None => Some(None),
                        },
                        None => None,
                    },
                    Err(_) => None,
                } {
                    Some(found) => found,
                    None => match image::open(path) {
                        Ok(image) => {
                            let image = image.to_rgba8();

                            match LOCAL_CACHE.write() {
                                Ok(mut cache) => {
                                    cache.insert(path.to_string(), Some(image.clone()));
                                    Some(resize(&image, size))
                                }
                                Err(_) => None,
                            }
                        }
                        Err(_) => match LOCAL_CACHE.write() {
                            Ok(mut cache) => {
                                cache.insert(path.to_string(), None);
                                None
                            }
                            Err(_) => None,
                        },
                    },
                }
            }
            EmojiPath::External { path, discord } => {
                if *discord && !RESOLVE_DISCORD {
                    return None;
                }

                EXTERNAL_CACHE.get_with_by_ref(path, || match CLIENT.get(path).send() {
                    Ok(response) => match response.bytes() {
                        Ok(bytes) => match image::load_from_memory(&bytes) {
                            Ok(image) => {
                                let image = image.to_rgba8();
                                Some(resize(&image, size))
                            }
                            Err(_) => None,
                        },
                        Err(_) => None,
                    },
                    Err(_) => None,
                })
            }
            EmojiPath::None => None,
        }
    }
}

impl<const RESOLVE_DISCORD: bool> EmojiResolver for DefaultEmojiResolver<RESOLVE_DISCORD> {
    fn resolve(&mut self, emojis: &Vec<UnresolvedEmoji>) -> Vec<ResolvedEmoji> {
        emojis
            .par_iter()
            .map(|emoji| ResolvedEmoji {
                image: Self::resolve_emoji(&emoji.path, emoji.size),
                id: emoji.id,
            })
            .collect()
    }
}
