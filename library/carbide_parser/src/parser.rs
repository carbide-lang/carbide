use crate::errors::CarbideParserError;
use crate::keywords::Keywords;
use crate::operators::{BinaryOperators, UnaryOperators};
use crate::tokens::{Token, Tokens};

pub struct CarbideParser<'a> {
    src: &'a str,
    pos: usize,
}

/// Attempt to cast a `u64` as a `usize`
/// 
/// # Errors
/// Returns `Err` if the u64 fails to cast to `usize`
#[inline(always)]
fn usize_from(v: u64) -> Result<usize, CarbideParserError> {
    usize::try_from(v)
        .map_err(|e| CarbideParserError::CastIntFailed(v.to_string(), "usize".to_string(), e))
}

impl<'a> CarbideParser<'a> {
    #[must_use]
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
                self.next();
                continue;
            }

            if ch.is_ascii_alphabetic() || ch == '_' {
                let token = self.parse_identifier(start)?;
                tokens.push(token);
                continue;
            }

            if ch.is_ascii_digit() {
                let token = self.parse_number(start)?;
                tokens.push(token);
                continue;
            }

            if BinaryOperators::starts_with(ch) || UnaryOperators::starts_with(ch) {
                let token = self.parse_operator(start)?;
                tokens.push(token);
                continue;
            }

            // Throw an error since parsing should catch everything
            return Err(CarbideParserError::UnexpectedChar(ch));
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
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            // Get a slice without `0x`
            let hex_digits = &self.src[(usize_from(start)? + 2)..usize_from(end)?];

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
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            // Get a slice without `0b`
            let hex_digits = &self.src[(usize_from(start)? + 2)..usize_from(end)?];

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
        let slice = &self.src[usize_from(start)?..usize_from(end)?];

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
    /// Attempts to parse an identifier
    ///
    /// # Errors
    /// Returns `Err` if parsing the identifier fails
    fn parse_identifier(&mut self, start: u64) -> Result<Token<'a>, CarbideParserError> {
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

impl<'a> CarbideParser<'a> {
    /// Attempts to parse an operator (`==`, `!=`, `!`, etc.)
    ///
    /// # Errors
    /// Returns `Err` if the operator is unrecognized
    pub fn parse_operator(&mut self, start: u64) -> Result<Token<'a>, CarbideParserError> {
        let mut op = String::new();

        if let Some(ch) = self.next() {
            op.push(ch);

            if let Some(next_ch) = self.peek() {
                let two_char = format!("{op}{next_ch}");
                if BinaryOperators::try_from(two_char.as_str()).is_ok() {
                    self.next();
                    op = two_char;
                }
            }
        } else {
            return Err(CarbideParserError::UnexpectedEOF);
        }

        let end = self.pos as u64;
        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        if let Ok(bin_op) = BinaryOperators::try_from(op.as_str()) {
            return Ok(Token {
                token_type: Tokens::BinaryOperator(bin_op),
                span: start..end,
                src: slice,
            });
        }

        if let Ok(un_op) = UnaryOperators::try_from(op.as_str()) {
            return Ok(Token {
                token_type: Tokens::UnaryOperator(un_op),
                span: start..end,
                src: slice,
            });
        }

        Err(CarbideParserError::UnexpectedChar(
            op.chars().next().ok_or(CarbideParserError::UnexpectedEOF)?,
        ))
    }
}
