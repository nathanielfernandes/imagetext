use once_cell::sync::Lazy;
use regex::Regex;
use std::iter::Peekable;

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
static EMOJI_UNICODE_RE_STR: Lazy<String> = Lazy::new(|| {
    let mut emojis = EMOJIS
        .iter()
        .map(|e| format!("{}", regex::escape(e.as_str())))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    emojis.join("|")
});
static EMOJI_SHORT_CODES_RE_STR: Lazy<String> = Lazy::new(|| {
    let mut emojis = EMOJIS
        .iter()
        .flat_map(|e| e.shortcodes())
        .map(|e| format!(":{}:", regex::escape(&e)))
        .collect::<Vec<_>>();

    emojis.sort_by(|a, b| b.len().cmp(&a.len()));
    emojis.join("|")
});
static DISCORD_EMOJI_RE_STR: &'static str = r"<a?:[a-zA-Z0-9_]{2, 32}:[0-9]{17,22}>";
static EMOJI_RE_STR: Lazy<String> = Lazy::new(|| {
    format!(
        "{}|{}|{}",
        EMOJI_UNICODE_RE_STR.as_str(),
        EMOJI_SHORT_CODES_RE_STR.as_str(),
        DISCORD_EMOJI_RE_STR
    )
});

static EMOJI_UNICODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&EMOJI_UNICODE_RE_STR).expect("Failed to compile emoji regex"));

static EMOJI_SHORT_CODES_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&EMOJI_SHORT_CODES_RE_STR).expect("Failed to compile emoji shortcode regex")
});

static DISCORD_EMOJI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<a?:[a-zA-Z0-9_]{2, 32}:([0-9]{17,22})>")
        .expect("Failed to compile discord emoji regex")
});

static TEXT_TOKEN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(r"{}|[\s\S]", EMOJI_RE_STR.as_str(),))
        .expect("Failed to compile emoji regex")
});

pub(crate) const PLACEHOLDER_EMOJI: char = '😀';
pub(crate) const PLACEHOLDER_EMOJI_STR: &'static str = "😀";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmojiType {
    Regular(&'static emojis::Emoji),
    Discord(u64),
}

impl EmojiType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmojiType::Regular(e) => e.as_str(),
            EmojiType::Discord(_) => PLACEHOLDER_EMOJI_STR,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Emoji(EmojiType),
    WhiteSpace,

    // Grapheme(String),
    Char(char),
}

pub struct TextTokenIterator<'t> {
    caps: Peekable<regex::CaptureMatches<'static, 't>>,

    parse_shortcodes: bool,
    parse_discord_emojis: bool,
}

impl<'t> TextTokenIterator<'t> {
    pub fn new(text: &'t str, parse_shortcodes: bool, parse_discord_emojis: bool) -> Self {
        Self {
            caps: TEXT_TOKEN_RE.captures_iter(text).peekable(),
            parse_shortcodes,
            parse_discord_emojis,
        }
    }

    fn parse_unicode_emoji(&self, s: &'t str) -> Option<EmojiType> {
        if EMOJI_UNICODE_RE.is_match(s) {
            if let Some(e) = emojis::get(s) {
                return Some(EmojiType::Regular(e));
            }
        }
        None
    }

    fn parse_discord_emoji(&self, s: &'t str) -> Option<EmojiType> {
        if self.parse_discord_emojis {
            if let Some(caps) = DISCORD_EMOJI_RE.captures(s) {
                if let Some(m) = caps.get(1) {
                    if let Ok(id) = m.as_str().parse::<u64>() {
                        return Some(EmojiType::Discord(id));
                    }
                }
            }
        }
        None
    }

    fn parse_shortcode_emoji(&self, s: &'t str) -> Option<EmojiType> {
        if self.parse_shortcodes && EMOJI_SHORT_CODES_RE.is_match(s) {
            if let Some(e) = emojis::get_by_shortcode(&s.replace(":", "")) {
                return Some(EmojiType::Regular(e));
            }
        }
        None
    }

    fn parse_emoji(&self, s: &'t str) -> Option<EmojiType> {
        self.parse_unicode_emoji(s)
            .or_else(|| self.parse_discord_emoji(s))
            .or_else(|| self.parse_shortcode_emoji(s))
    }

    #[inline]
    fn is_whitespace(&self, s: &'t str) -> bool {
        s.chars().all(|c| c.is_whitespace())
    }
}

impl<'t> Iterator for TextTokenIterator<'t> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(caps) = self.caps.next() {
            if let Some(cap) = caps.get(0) {
                let s = cap.as_str();

                if self.is_whitespace(s) {
                    return Some(Token::WhiteSpace);
                }

                if let Some(e) = self.parse_emoji(s) {
                    return Some(Token::Emoji(e));
                }

                // continue parsing as Text
                let mut text = String::from(s);
                while let Some(caps) = self.caps.peek() {
                    if let Some(cap) = caps.get(0) {
                        let s = cap.as_str();

                        if self.is_whitespace(s) {
                            break;
                        }

                        if let Some(_) = self.parse_emoji(s) {
                            break;
                        }

                        text.push_str(s);
                        self.caps.next();
                    }
                }

                return Some(Token::Word(text));
            }
        }
        None
    }
}

pub trait Tokens {
    fn string(&self) -> String;
    fn chars(&self) -> Vec<Token>;
    fn chars_and_emojis(&self) -> Vec<Token>;
}

impl Tokens for Vec<Token> {
    fn string(&self) -> String {
        let mut s = String::new();
        for token in self {
            match token {
                Token::Emoji(e) => s.push_str(e.as_str()),
                Token::Word(w) => s.push_str(w),
                // Token::Grapheme(g) => s.push_str(g),
                Token::Char(c) => s.push(*c),
                Token::WhiteSpace => s.push(' '),
            }
        }
        s
    }

    fn chars(&self) -> Vec<Token> {
        let mut chars = Vec::new();
        for token in self {
            match token {
                Token::Emoji(e) => chars.extend(e.as_str().chars().map(Token::Char)),
                Token::Word(w) => chars.extend(w.chars().map(Token::Char)),
                // Token::Grapheme(g) => chars.extend(g.chars().map(Token::Char)),
                Token::Char(c) => chars.push(Token::Char(*c)),
                Token::WhiteSpace => chars.push(Token::Char(' ')),
            }
        }
        chars
    }

    fn chars_and_emojis(&self) -> Vec<Token> {
        let mut chars = Vec::new();
        for token in self {
            match token {
                Token::Emoji(e) => chars.push(Token::Emoji(e.clone())),
                Token::Word(w) => chars.extend(w.chars().map(Token::Char)),
                // Token::Grapheme(g) => chars.extend(g.chars().map(Token::Char)),
                Token::Char(c) => chars.push(Token::Char(*c)),
                Token::WhiteSpace => chars.push(Token::Char(' ')),
            }
        }
        chars
    }
}

#[test]
fn test_text_token_iterator() {
    let text = "<:notlikecat:739615190525542482>\nsssss:smile:ssssss😈ssssssss<:notlikecat:739615190525542482>sssssssssssssssssssssssss😈ssssssss<:notlikecat:739615190525542482>ssss<:notlikecat:739615190525542482>               rogramming language is an A <:notlikecat:739615190525542482><:notlikecat:739615190525542482> rogramming language is an A <:notlikecat:739615190525542482><:notlikecat:739615190525542482> rogramming language is an A <:notlikecat:739615190525542482><:notlikecat:739615190525542482> rogramming language is an ";

    let tokens: Vec<Token> = TextTokenIterator::new(text, true, true).collect();

    let start = std::time::Instant::now();
    let tokens: Vec<Token> = TextTokenIterator::new(text, true, true).collect();
    // for token in CharTokenIterator::new(&tokens) {
    //     println!("{:?}", token);
    // }

    // for token in tokens.chars() {
    //     println!("{:?}", token);
    // }

    println!("Time: {:?}", start.elapsed());
}
