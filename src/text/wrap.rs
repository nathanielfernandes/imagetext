use crate::prelude::SuperFont;

use super::{
    measure::tokens_width,
    parse::{Token, Tokens},
};

#[derive(Debug, Clone, Copy)]
pub enum WrapStyle {
    Word,
    Character,
}

pub fn text_wrap_tokens(
    tokens: impl Iterator<Item = Token>,
    width: i32,
    font: &SuperFont,
    scale: rusttype::Scale,
    style: WrapStyle,
) -> Vec<Vec<Token>> {
    match style {
        WrapStyle::Word => word_wrap_tokens(
            tokens.filter(|t| !matches!(t, Token::WhiteSpace)),
            width,
            font,
            scale,
            false,
        ),
        WrapStyle::Character => character_wrap_tokens(tokens, width, font, scale),
    }
}

pub(crate) fn word_wrap_tokens(
    tokens: impl Iterator<Item = Token>,
    width: i32,
    font: &SuperFont,
    scale: rusttype::Scale,
    no_whitespace: bool,
) -> Vec<Vec<Token>> {
    let mut result = Vec::new();

    let mut line = Vec::new();

    for word in tokens {
        let mut new_line = line.clone();
        if !no_whitespace && !new_line.is_empty() {
            new_line.push(Token::WhiteSpace);
        }
        new_line.push(word.clone());

        let w = tokens_width(scale, font, &new_line);

        if w > width {
            if !line.is_empty() {
                result.push(line);
            }
            line = vec![word];
        } else {
            line = new_line;
        }
    }

    line = line
        .into_iter()
        .filter(|t| !matches!(t, Token::WhiteSpace))
        .collect();

    if !line.is_empty() {
        result.push(line);
    }

    result
}

pub(crate) fn character_wrap_tokens(
    tokens: impl Iterator<Item = Token>,
    width: i32,
    font: &SuperFont,
    scale: rusttype::Scale,
) -> Vec<Vec<Token>> {
    let mut result = Vec::new();

    for line in word_wrap_tokens(
        tokens.filter(|t| !matches!(t, Token::WhiteSpace)),
        width,
        font,
        scale,
        false,
    ) {
        let w = tokens_width(scale, font, &line);

        if w > width {
            word_wrap_tokens(line.chars().into_iter(), width, font, scale, true)
                .into_iter()
                .for_each(|l| {
                    result.push(l);
                });
        } else {
            result.push(line);
        }
    }

    word_wrap_tokens(
        result.into_iter().flat_map(|line| {
            let mut result = Vec::new();
            // turn chars into words

            let mut buffer = String::new();
            for token in line {
                match token {
                    Token::Word(word) => {
                        if !buffer.is_empty() {
                            result.push(Token::Word(buffer));
                            buffer = String::new();
                        }
                        result.push(Token::Word(word))
                    }
                    Token::WhiteSpace => {
                        if !buffer.is_empty() {
                            result.push(Token::Word(buffer));
                            buffer = String::new();
                        }
                    }
                    Token::Emoji(emoji) => {
                        if !buffer.is_empty() {
                            result.push(Token::Word(buffer));
                            buffer = String::new();
                        }
                        result.push(Token::Emoji(emoji));
                    }
                    Token::Char(c) => {
                        buffer.push(c);
                    }
                }
            }

            if !buffer.is_empty() {
                result.push(Token::Word(buffer));
            }

            result
        }),
        width,
        font,
        scale,
        false,
    )
}
