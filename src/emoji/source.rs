use image::RgbaImage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmojiType {
    Regular(&'static emojis::Emoji),
    Discord(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmojiPath {
    Local(String),
    External { path: String, discord: bool },
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EmojiSource {
    Dir(String), // Path to directory

    Twitter,
    Apple,
    Google,
    Microsoft,
    Samsung,
    WhatsApp,
    // Facebook, NOT WORKING
    JoyPixels,
    OpenMoji,
    Emojidex,
    Messenger,
    Mozilla,
    Lg,
    Htc,

    Twemoji,
}

impl EmojiSource {
    const EMOJI_CDN: &'static str = "https://emojicdn.elk.sh";
    const DISCORD_EMOJI_CDN: &'static str = "https://cdn.discordapp.com/emojis";

    pub fn emoji_file_name(emoji: &str) -> String {
        let mut e = String::new();
        for c in {
            if emoji.contains('\u{200d}') {
                emoji.to_string()
            } else {
                emoji.replace("\u{fe0f}", "")
            }
        }
        .chars()
        {
            e.push_str(&format!("{:x}", c as u32));
            e.push('-');
        }
        e.pop();

        e.push_str(".png");

        e
    }

    pub fn style(&self) -> &'static str {
        match self {
            EmojiSource::Twitter | EmojiSource::Twemoji => "twitter",
            EmojiSource::Apple => "apple",
            EmojiSource::Google => "google",
            EmojiSource::Microsoft => "microsoft",
            EmojiSource::Samsung => "samsung",
            EmojiSource::WhatsApp => "whatsapp",
            // EmojiSource::Facebook => "facebook",
            EmojiSource::JoyPixels => "joypixels",
            EmojiSource::OpenMoji => "openmoji",
            EmojiSource::Emojidex => "emojidex",
            EmojiSource::Mozilla => "mozilla",
            EmojiSource::Messenger => "messenger",
            EmojiSource::Lg => "lg",
            EmojiSource::Htc => "htc",

            EmojiSource::Dir(_) => "local",
        }
    }

    pub fn build_path(&self, emoji: &EmojiType, discord: bool) -> EmojiPath {
        match emoji {
            EmojiType::Regular(e) => match self {
                EmojiSource::Dir(path) => {
                    EmojiPath::Local(format!("{}/{}", path, Self::emoji_file_name(e.as_str())))
                }
                _ => EmojiPath::External {
                    path: format!("{}/{}?style={}", Self::EMOJI_CDN, e.as_str(), self.style()),
                    discord: false,
                },
            },
            EmojiType::Discord(id) => {
                if discord {
                    EmojiPath::External {
                        path: format!("{}/{}.png", Self::DISCORD_EMOJI_CDN, id),
                        discord: true,
                    }
                } else {
                    EmojiPath::None
                }
            }
        }
    }
}

pub struct UnresolvedEmoji {
    pub path: EmojiPath,
    pub size: u32,
    pub id: usize,
}

pub struct ResolvedEmoji {
    pub image: Option<RgbaImage>,
    pub id: usize,
}

pub trait EmojiResolver {
    fn resolve(&mut self, emojis: &Vec<UnresolvedEmoji>) -> Vec<ResolvedEmoji>;
}

#[test]
pub fn emoji_src() {
    let src = EmojiSource::Dir(String::from("test"));
    let emoji = EmojiType::Regular(emojis::get("😀").unwrap());
    assert_eq!(
        src.build_path(&emoji, false),
        EmojiPath::Local("test/1f600.png".to_string())
    );

    let src = EmojiSource::Twemoji;
    let emoji = EmojiType::Discord(1234567890);
    assert_eq!(
        src.build_path(&emoji, true),
        EmojiPath::External {
            path: "https://cdn.discordapp.com/emojis/1234567890.png".to_string(),
            discord: true
        }
    );

    let emoji = EmojiType::Regular(emojis::get("😀").unwrap());
    assert_eq!(
        src.build_path(&emoji, false),
        EmojiPath::External {
            path: "https://emojicdn.elk.sh/😀?style=twemoji".to_string(),
            discord: false
        }
    );

    let src = EmojiSource::Twitter;
    let emoji = EmojiType::Regular(emojis::get("😀").unwrap());
    assert_eq!(
        src.build_path(&emoji, false),
        EmojiPath::External {
            path: "https://emojicdn.elk.sh/😀?style=twitter".to_string(),
            discord: false
        }
    );

    let emoji = EmojiType::Discord(1234567890);
    assert_eq!(src.build_path(&emoji, false), EmojiPath::None);
}
