use crate::errors::CarbideParserError;
use crate::tokens::{Token, Tokens};

pub struct CarbideParser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> CarbideParser<'a> {
    pub fn from_src(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn consume_while<F: FnMut(char) -> bool>(&mut self, mut cond: F) {
        while let Some(ch) = self.peek() {
            if cond(ch) {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Attempt to parse the source into a list of [`Tokens`][Token]
    ///
    /// # Errors
    /// Returns `Err` if parsing the source fails
    pub fn parse(&mut self) -> Result<Vec<Token<'a>>, CarbideParserError> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            let start = self.pos as u64;
            let ch = self.peek().ok_or(CarbideParserError::UnexpectedEOF)?;

            if !ch.is_ascii() {
                return Err(CarbideParserError::NonASCIIChar(ch));
            }

            if ch.is_ascii_whitespace() {
                self.consume_while(|c| c.is_ascii_whitespace());
                let end = self.pos as u64;
                let slice = &self.src[start as usize..end as usize];
                tokens.push(Token {
                    token_type: Tokens::Whitespace,
                    span: start..end,
                    src: slice,
                });
                continue;
            }

            // Entry point for identifier parsing
            if ch.is_ascii_alphabetic() || ch == '_' {
                let token = self.parse_identifier(start);
                tokens.push(token);
                continue;
            }

            if ch.is_ascii_digit() {
                let token = self.parse_number(start)?;
                tokens.push(token);
                continue;
            }

            // TODO: Throw err
            self.next();
        }

        Ok(tokens)
    }
}

impl<'a> CarbideParser<'a> {
    /// Attempt to parse a number [`Token`]
    ///
    /// # Errors
    /// Returns `Err` if fails
    fn parse_number(&mut self, start: u64) -> Result<Token<'a>, CarbideParserError> {
        if self.src[self.pos..].starts_with("0x") {
            self.pos += 2; // consume `0x`
            self.consume_while(|c| c.is_ascii_hexdigit());

            let end = self.pos as u64;
            let slice = &self.src[start as usize..end as usize];

            // Get a slice without `0x`
            let hex_digits = &self.src[(start as usize + 2)..end as usize];

            return Ok(Token {
                token_type: Tokens::HexLiteral(i64::from_str_radix(hex_digits, 16).map_err(
                    |e| CarbideParserError::InvalidHexLiteral(hex_digits.to_string(), e),
                )?),
                span: start..end,
                src: slice,
            });
        }

        if self.src[self.pos..].starts_with("0b") {
            self.pos += 2; // consume `0b`
            self.consume_while(|c| c == '0' || c == '1');

            let end = self.pos as u64;
            let slice = &self.src[start as usize..end as usize];

            // Get a slice without `0b`
            let hex_digits = &self.src[(start as usize + 2)..end as usize];

            return Ok(Token {
                token_type: Tokens::BinaryLiteral(i64::from_str_radix(hex_digits, 2).map_err(
                    |e| CarbideParserError::InvalidBinaryLiteral(hex_digits.to_string(), e),
                )?),
                span: start..end,
                src: slice,
            });
        }

        let mut has_dot = false;
        self.consume_while(|c| {
            if c == '.' {
                if has_dot {
                    false
                } else {
                    has_dot = true;
                    true
                }
            } else {
                c.is_ascii_digit()
            }
        });

        let end = self.pos as u64;
        let slice = &self.src[start as usize..end as usize];

        if has_dot {
            Ok(Token {
                token_type: Tokens::FloatLiteral(
                    slice.parse::<f64>().map_err(|e| {
                        CarbideParserError::InvalidFloatLiteral(slice.to_string(), e)
                    })?,
                ),
                span: start..end,
                src: slice,
            })
        } else {
            Ok(Token {
                token_type: Tokens::IntLiteral(slice.parse::<i64>().map_err(|e| {
                    CarbideParserError::InvalidIntegerLiteral(slice.to_string(), e)
                })?),
                span: start..end,
                src: slice,
            })
        }
    }
}

impl<'a> CarbideParser<'a> {
    /// Parses an indentifier, consumes
    fn parse_identifier(&mut self, start: u64) -> Token<'a> {
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
        let end = self.pos as u64;
        let slice = &self.src[start as usize..end as usize];

        Token {
            token_type: Tokens::Identifier(slice),
            span: start..end,
            src: slice,
        }
    }
}
