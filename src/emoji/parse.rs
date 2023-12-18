use hashbrown::{HashMap, HashSet};
use once_cell::sync::Lazy;
use regex::Regex;

use super::source::EmojiType;

static EMOJIS: Lazy<Vec<&'static emojis::Emoji>> = Lazy::new(|| {
    let mut emojis = emojis::iter().collect::<Vec<_>>();
    let mut emojis_tones = Vec::new();
    for emoji in emojis.iter() {
        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::Light) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::MediumLight) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::Medium) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::MediumDark) {
            emojis_tones.push(e);
        }

        if let Some(e) = emoji.with_skin_tone(emojis::SkinTone::Dark) {
            emojis_tones.push(e);
        }
    }
    emojis.extend(emojis_tones);

    emojis
});

static EMOJI_SHORTCODE_MAP: Lazy<HashMap<String, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for emoji in EMOJIS.iter() {
        for s in emoji.shortcodes() {
            map.insert(format!(":{}:", s), emoji.as_str());
        }
    }
    map
});

static EMOJI_UNICODE_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut map = HashSet::new();
    for emoji in EMOJIS.iter() {
        map.insert(emoji.as_str());
    }
    map
});

static EMOJI_MAP: Lazy<HashMap<String, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for emoji in EMOJIS.iter() {
        for s in emoji.shortcodes() {
            map.insert(format!(":{}:", s), emoji.as_str());
        }
    }

    for emoji in EMOJIS.iter() {
        map.insert(emoji.as_str().to_string(), emoji.as_str());
    }

    map
});

static EMOJI_UNICODE_RE_STR: Lazy<String> = Lazy::new(|| {
    let mut emojis = emojis::iter()
        .map(|e| format!("{}", regex::escape(e.as_str())))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    format!("{}", emojis.join("|"))
});

static EMOJI_SHORT_CODES_RE_STR: &str = r":[a-zA-Z0-9_\+\\-]{1,32}:";
// Lazy::new(|| {
// let mut emojis = EMOJIS
//     .iter()
//     .flat_map(|e| e.shortcodes())
//     .map(|s| format!(":{}:", regex::escape(s)))
//     .collect::<Vec<_>>();
// emojis.sort_by(|a, b| b.len().cmp(&a.len()));
// format!("{}", emojis.join("|"))
// });

static DISCORD_EMOJI_RE_STR: &str = r"<a?:[a-zA-Z0-9_]{2, 32}:[0-9]{17,22}>";
static EMOJI_RE_STR: Lazy<String> = Lazy::new(|| {
    format!(
        "{}|{}|{}",
        EMOJI_UNICODE_RE_STR.as_str(),
        EMOJI_SHORT_CODES_RE_STR,
        DISCORD_EMOJI_RE_STR
    )
});

static EMOJI_UNICODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&EMOJI_UNICODE_RE_STR).expect("Failed to compile emoji regex"));

static EMOJI_SHORT_CODES_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&EMOJI_SHORT_CODES_RE_STR).expect("Failed to compile emoji shortcode regex")
});

static DISCORD_EMOJI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<a?:[a-zA-Z0-9_]{2, 32}:[0-9]{17,22}>")
        .expect("Failed to compile discord emoji regex")
});

static TEXT_TOKEN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(r"{}|[\s\S]", EMOJI_RE_STR.as_str(),))
        .expect("Failed to compile emoji regex")
});

pub fn build_regex() {
    let _ = &*EMOJIS;
    let _ = &*EMOJI_MAP;
    let _ = &*EMOJI_UNICODE_SET;
    let _ = &*EMOJI_SHORTCODE_MAP;
    let _ = &*EMOJI_UNICODE_RE;
    let _ = &*EMOJI_SHORT_CODES_RE;
    let _ = &*DISCORD_EMOJI_RE;
    let _ = &*TEXT_TOKEN_RE;
}

pub(crate) const PLACEHOLDER_EMOJI: char = '😀';

#[inline(always)]
fn parse_unicode_emoji(s: &str) -> Option<EmojiType> {
    if let Some(e) = EMOJI_UNICODE_SET.get(s) {
        return Some(EmojiType::Regular(e));
    }
    None
}

#[inline(always)]
fn parse_discord_emoji(s: &str) -> Option<EmojiType> {
    if DISCORD_EMOJI_RE.is_match(s) {
        if let Some(m) = s.split(':').nth(2) {
            if let Ok(id) = m[..m.len() - 1].parse::<u64>() {
                return Some(EmojiType::Discord(id));
            }
        }
    }
    None
}

// #[inline(always)]
// fn parse_shortcode_emoji(s: &str) -> Option<EmojiType> {
//     if let Some(e) = EMOJI_SHORTCODE_MAP.get(s) {
//         return Some(EmojiType::Regular(e));
//     }

//     None
// }

#[inline(always)]
fn parse_emoji(s: &str) -> Option<EmojiType> {
    if let Some(e) = EMOJI_MAP.get(s) {
        return Some(EmojiType::Regular(e));
    }

    None
}

pub fn parse_out_emojis<'t>(
    text: &'t str,
    parse_shortcodes: bool,
    parse_discord_emojis: bool,
) -> (String, Vec<EmojiType>) {
    let mut parsed = String::with_capacity(text.len());
    let mut emojis = Vec::new();

    match (parse_shortcodes, parse_discord_emojis) {
        (false, false) => {
            for cap in TEXT_TOKEN_RE.find_iter(text) {
                let s = cap.as_str();
                if let Some(emoji) = parse_unicode_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }
                parsed.push_str(s);
            }
            return (parsed, emojis);
        }
        (true, false) => {
            for cap in TEXT_TOKEN_RE.find_iter(text) {
                let s = cap.as_str();
                if let Some(emoji) = parse_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }
                parsed.push_str(s);
            }
            return (parsed, emojis);
        }
        (false, true) => {
            for cap in TEXT_TOKEN_RE.find_iter(text) {
                let s = cap.as_str();

                if let Some(emoji) = parse_unicode_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }

                if let Some(emoji) = parse_discord_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }

                parsed.push_str(s);
            }
            return (parsed, emojis);
        }
        (true, true) => {
            for cap in TEXT_TOKEN_RE.find_iter(text) {
                let s = cap.as_str();

                if let Some(emoji) = parse_emoji(s) {
                    emojis.push(emoji);
                    parsed.push(PLACEHOLDER_EMOJI);
                    continue;
                }

                if parse_discord_emojis {
                    if let Some(emoji) = parse_discord_emoji(s) {
                        emojis.push(emoji);
                        parsed.push(PLACEHOLDER_EMOJI);
                        continue;
                    }
                }

                parsed.push_str(s);
            }
        }
    }

    (parsed, emojis)
}

pub fn clean_emojis(text: &str) -> String {
    let mut parsed = String::with_capacity(text.len());

    for cap in TEXT_TOKEN_RE.find_iter(text) {
        let s = cap.as_str();

        if let Some(_) = parse_emoji(s) {
            parsed.push(PLACEHOLDER_EMOJI);
            continue;
        }

        if let Some(_) = parse_discord_emoji(s) {
            parsed.push(PLACEHOLDER_EMOJI);
            continue;
        }

        parsed.push_str(s);
    }

    parsed
}

#[test]
fn parse_out_emojis_test() {
    let text = "h😨😰 my 😓 n🐢ame i☕s 会のすべ aての構成員 nathan and i drin😳🥵 boop coop, the quick brown fox jumps over the lazy dog";
    let (s, emojis) = parse_out_emojis(text, false, false);

    assert_eq!(s, "h😀😀 my 😀 n😀ame i😀s 会のすべ aての構成員 nathan and i drin😀😀 boop coop, the quick brown fox jumps over the lazy dog");
    assert_eq!(
        emojis,
        vec![
            EmojiType::Regular(emojis::get("😨").unwrap().as_str()),
            EmojiType::Regular(emojis::get("😰").unwrap().as_str()),
            EmojiType::Regular(emojis::get("😓").unwrap().as_str()),
            EmojiType::Regular(emojis::get("🐢").unwrap().as_str()),
            EmojiType::Regular(emojis::get("☕").unwrap().as_str()),
            EmojiType::Regular(emojis::get("😳").unwrap().as_str()),
            EmojiType::Regular(emojis::get("🥵").unwrap().as_str()),
        ]
    );

    // shortcode test
    let text = "hello :smile: soup :pensive:";
    let (s, emojis) = parse_out_emojis(text, true, false);

    assert_eq!(s, "hello 😀 soup 😀");

    assert_eq!(
        emojis,
        vec![
            EmojiType::Regular(emojis::get_by_shortcode("smile").unwrap().as_str()),
            EmojiType::Regular(emojis::get_by_shortcode("pensive").unwrap().as_str()),
        ]
    );

    // discord test
    let text = "hello <a:smile:123456789012345678> soup :sob: <:soup:123456999012345678>";

    let (s, emojis) = parse_out_emojis(text, true, true);

    assert_eq!(s, "hello 😀 soup 😀 😀");

    assert_eq!(
        emojis,
        vec![
            EmojiType::Discord(123456789012345678),
            EmojiType::Regular(emojis::get_by_shortcode("sob").unwrap().as_str()),
            EmojiType::Discord(123456999012345678),
        ]
    );
}
