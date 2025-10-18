use crate::errors::CarbideLexerError;
use crate::keywords::Keywords;
use crate::operators::{BinaryOperators, UnaryOperators};
use crate::tokens::{Token, Tokens};

pub struct CarbideLexer<'a> {
    src: &'a str,
    pos: usize,
}

/// Attempt to cast a `u64` as a `usize`
///
/// # Errors
/// Returns `Err` if the u64 fails to cast to `usize`
#[inline]
fn usize_from(v: u64) -> Result<usize, CarbideLexerError> {
    usize::try_from(v)
        .map_err(|e| CarbideLexerError::CastIntFailed(v.to_string(), "usize".to_string(), e))
}

impl<'a> CarbideLexer<'a> {
    #[must_use]
    pub fn from_src(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    /// Check if `pos` is at the EOI
    #[inline]
    fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    /// Get the next char in `src`
    #[inline]
    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    /// If there is a next char, increment pos
    #[inline]
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    /// Consume chars while the `cond` is valid
    #[inline]
    fn consume_while<F: FnMut(char) -> bool>(&mut self, mut cond: F) {
        while let Some(ch) = self.peek() {
            if cond(ch) {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Skip over whitespace and comments
    fn skip_whitespace_and_comments(&mut self) -> Result<(), CarbideLexerError> {
        loop {
            if let Some(ch) = self.peek() {
                if ch.is_ascii_whitespace() {
                    self.next();
                    continue;
                }
            }

            if self.src[self.pos..].starts_with("//") {
                self.pos += 2;
                self.consume_while(|c| c != '\n');
                continue;
            }

            if self.src[self.pos..].starts_with("/*") {
                self.skip_nested_comment()?;
                continue;
            }

            break;
        }

        return Ok(());
    }

    fn skip_nested_comment(&mut self) -> Result<(), CarbideLexerError> {
        self.pos += 2;
        let mut depth = 1;

        while !self.is_eof() && depth > 0 {
            if self.src[self.pos..].starts_with("/*") {
                self.pos += 2;
                depth += 1;
            } else if self.src[self.pos..].starts_with("*/") {
                self.pos += 2;
                depth -= 1;
            } else {
                self.next();
            }
        }

        if depth > 0 {
            return Err(CarbideLexerError::UnclosedComment);
        }

        return Ok(());
    }

    /// Attempt to lex the source into a list of [`Tokens`][Token]
    ///
    /// # Errors
    /// Returns `Err` if parsing the source fails
    pub fn lex(&mut self) -> Result<Vec<Token<'a>>, CarbideLexerError> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            self.skip_whitespace_and_comments()?;

            if self.is_eof() {
                break;
            }

            let start = self.pos as u64;
            let ch = self.peek().ok_or(CarbideLexerError::UnexpectedEOF)?;

            if !ch.is_ascii() {
                return Err(CarbideLexerError::NonASCIIChar(ch));
            }

            if ch.is_ascii_alphabetic() || ch == '_' {
                let token = self.lex_identifier(start)?;
                tokens.push(token);
                continue;
            }

            if ch.is_ascii_digit() {
                let token = self.lex_number(start)?;
                tokens.push(token);
                continue;
            }

            if BinaryOperators::starts_with(ch) || UnaryOperators::starts_with(ch) {
                let token = self.lex_operator(start)?;
                tokens.push(token);
                continue;
            }

            if let Some(token) = self.lex_single_char(start)? {
                tokens.push(token);
                continue;
            }

            // Throw an error since lexing should catch everything
            return Err(CarbideLexerError::UnexpectedChar(ch));
        }

        Ok(tokens)
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempt to lex a number [`Token`]
    ///
    /// # Errors
    /// Returns `Err` if fails
    fn lex_number(&mut self, start: u64) -> Result<Token<'a>, CarbideLexerError> {
        if self.src[self.pos..].starts_with("0x") {
            self.pos += 2;
            self.consume_while(|c| c.is_ascii_hexdigit());

            let end = self.pos as u64;
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            let hex_digits = &self.src[(usize_from(start)? + 2)..usize_from(end)?];

            return Ok(Token {
                token_type: Tokens::HexLiteral(i64::from_str_radix(hex_digits, 16).map_err(
                    |e| CarbideLexerError::InvalidHexLiteral(hex_digits.to_string(), e),
                )?),
                span: start..end,
                src: slice,
            });
        }

        if self.src[self.pos..].starts_with("0b") {
            self.pos += 2;
            self.consume_while(|c| c == '0' || c == '1');

            let end = self.pos as u64;
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            let hex_digits = &self.src[(usize_from(start)? + 2)..usize_from(end)?];

            return Ok(Token {
                token_type: Tokens::BinaryLiteral(i64::from_str_radix(hex_digits, 2).map_err(
                    |e| CarbideLexerError::InvalidBinaryLiteral(hex_digits.to_string(), e),
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
        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        if has_dot {
            Ok(Token {
                token_type: Tokens::FloatLiteral(
                    slice.parse::<f64>().map_err(|e| {
                        CarbideLexerError::InvalidFloatLiteral(slice.to_string(), e)
                    })?,
                ),
                span: start..end,
                src: slice,
            })
        } else {
            Ok(Token {
                token_type: Tokens::IntLiteral(
                    slice.parse::<i64>().map_err(|e| {
                        CarbideLexerError::InvalidIntegerLiteral(slice.to_string(), e)
                    })?,
                ),
                span: start..end,
                src: slice,
            })
        }
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempts to lex an identifier
    ///
    /// # Errors
    /// Returns `Err` if parsing the identifier fails
    fn lex_identifier(&mut self, start: u64) -> Result<Token<'a>, CarbideLexerError> {
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
        let end = self.pos as u64;

        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        let token_type = if let Ok(keyword) = Keywords::try_from(slice) {
            Tokens::Keyword(keyword)
        } else {
            Tokens::Identifier(slice)
        };

        Ok(Token {
            token_type,
            span: start..end,
            src: slice,
        })
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempts to lex an operator (`==`, `!=`, `!`, `=`, etc.)
    ///
    /// # Errors
    /// Returns `Err` if the operator is unrecognized
    pub fn lex_operator(&mut self, start: u64) -> Result<Token<'a>, CarbideLexerError> {
        let first_ch = self.next().ok_or(CarbideLexerError::UnexpectedEOF)?;

        if let Some(second_ch) = self.peek() {
            let two_char = format!("{first_ch}{second_ch}");

            if BinaryOperators::try_from(two_char.as_str()).is_ok() {
                self.next();
                let end = self.pos as u64;
                let slice = &self.src[usize_from(start)?..usize_from(end)?];

                return Ok(Token {
                    token_type: Tokens::BinaryOperator(BinaryOperators::try_from(slice)?),
                    span: start..end,
                    src: slice,
                });
            }

            if UnaryOperators::try_from(two_char.as_str()).is_ok() {
                self.next();
                let end = self.pos as u64;
                let slice = &self.src[usize_from(start)?..usize_from(end)?];

                return Ok(Token {
                    token_type: Tokens::UnaryOperator(UnaryOperators::try_from(slice)?),
                    span: start..end,
                    src: slice,
                });
            }
        }

        let end = self.pos as u64;
        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        if let Ok(bin_op) = BinaryOperators::try_from(slice) {
            return Ok(Token {
                token_type: Tokens::BinaryOperator(bin_op),
                span: start..end,
                src: slice,
            });
        }

        if let Ok(un_op) = UnaryOperators::try_from(slice) {
            return Ok(Token {
                token_type: Tokens::UnaryOperator(un_op),
                span: start..end,
                src: slice,
            });
        }

        Err(CarbideLexerError::UnexpectedChar(first_ch))
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempt to lex a single-character token (`()[]{},;:`)
    ///
    /// # Errors
    /// Returns `Err` if parsing the source fails
    fn lex_single_char(&mut self, start: u64) -> Result<Option<Token<'a>>, CarbideLexerError> {
        if let Some(ch) = self.peek()
            && Tokens::starts_with(ch)
        {
            self.next();
            let end = self.pos as u64;
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            if let Some(token_type) = Tokens::from_char(ch) {
                return Ok(Some(Token {
                    token_type,
                    span: start..end,
                    src: slice,
                }));
            }
        }
        Ok(None)
    }
}
