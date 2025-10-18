use crate::errors::CarbideLexerError;
use crate::keywords::Keywords;
use crate::operators::{BinaryOperators, UnaryOperators};
use crate::tokens::{SourceLocation, StringPart, Token, Tokens};

pub struct CarbideLexer<'a> {
    src: &'a str,
    pos: usize,
    line: u64,
    column: u64,
}

/// Result type that includes both successful tokens and errors
pub struct LexResult<'a> {
    pub tokens: Vec<Token<'a>>,
    pub errors: Vec<CarbideLexerError>,
}

impl LexResult<'_> {
    /// Returns true if lexing succeeded without errors
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns true if there were any errors
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
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
        Self {
            src,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get current source location
    #[inline]
    fn current_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.pos as u64,
        }
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

    /// If there is a next char, increment `pos` and update `line`/`column`
    #[inline]
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

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
            if let Some(ch) = self.peek()
                && ch.is_ascii_whitespace()
            {
                self.next();
                continue;
            }

            if self.src[self.pos..].starts_with("//") {
                self.pos += 2;
                self.column += 2;
                self.consume_while(|c| c != '\n');
                continue;
            }

            if self.src[self.pos..].starts_with("/*") {
                self.skip_nested_comment()?;
                continue;
            }

            break;
        }

        Ok(())
    }

    /// Skips nested comments
    fn skip_nested_comment(&mut self) -> Result<(), CarbideLexerError> {
        let start_loc = self.current_location();
        self.pos += 2;
        self.column += 2;
        let mut depth = 1;

        while !self.is_eof() && depth > 0 {
            if self.src[self.pos..].starts_with("/*") {
                self.pos += 2;
                self.column += 2;
                depth += 1;
            } else if self.src[self.pos..].starts_with("*/") {
                self.pos += 2;
                self.column += 2;
                depth -= 1;
            } else {
                self.next();
            }
        }

        if depth > 0 {
            return Err(CarbideLexerError::UnclosedComment(start_loc));
        }

        Ok(())
    }

    /// Attempt to recover from a lexer error by skipping to the next valid token start
    fn recover_from_error(&mut self) {
        self.next();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_whitespace()
                || ch.is_ascii_alphabetic()
                || ch == '_'
                || ch.is_ascii_digit()
                || BinaryOperators::starts_with(ch)
                || UnaryOperators::starts_with(ch)
                || Tokens::starts_with(ch)
                || ch == '/'
            {
                break;
            }
            self.next();
        }
    }

    /// Attempt to lex the source into a list of [`Tokens`][Token] with error recovery
    ///
    /// This method will attempt to recover from errors and continue lexing,
    /// collecting both valid tokens and errors encountered.
    pub fn lex(&mut self) -> LexResult<'a> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while !self.is_eof() {
            if let Err(e) = self.skip_whitespace_and_comments() {
                errors.push(e);
                self.recover_from_error();
                continue;
            }

            if self.is_eof() {
                break;
            }

            let start = self.pos as u64;
            let start_loc = self.current_location();

            let Some(ch) = self.peek() else {
                break;
            };

            if !ch.is_ascii() {
                errors.push(CarbideLexerError::NonASCIIChar(ch, start_loc));
                self.recover_from_error();
                continue;
            }

            if ch.is_ascii_alphabetic() || ch == '_' {
                match self.lex_identifier(start, start_loc) {
                    Ok(token) => tokens.push(token),
                    Err(e) => {
                        errors.push(e);
                        self.recover_from_error();
                    }
                }
                continue;
            }

            if ch == '"' {
                match self.lex_string(start, start_loc) {
                    Ok(Some(t)) => tokens.push(t),
                    Ok(None) => {}
                    Err(e) => {
                        errors.push(e);
                        self.recover_from_error();
                        continue;
                    }
                }
                continue;
            }

            if ch.is_ascii_digit() {
                match self.lex_number(start, start_loc) {
                    Ok(token) => tokens.push(token),
                    Err(e) => {
                        errors.push(e);
                        self.recover_from_error();
                    }
                }
                continue;
            }

            if BinaryOperators::starts_with(ch) || UnaryOperators::starts_with(ch) {
                match self.lex_operator(start, start_loc) {
                    Ok(token) => tokens.push(token),
                    Err(e) => {
                        errors.push(e);
                        self.recover_from_error();
                    }
                }
                continue;
            }

            match self.lex_single_char(start, start_loc) {
                Ok(Some(token)) => {
                    tokens.push(token);
                    continue;
                }
                Ok(None) => {}
                Err(e) => {
                    errors.push(e);
                    self.recover_from_error();
                    continue;
                }
            }

            errors.push(CarbideLexerError::UnexpectedChar(ch, start_loc));
            self.recover_from_error();
        }

        LexResult { tokens, errors }
    }

    /// [`CarbideLexer::lex()`] with the condition that it exits as soon as an error is found
    ///
    /// # Errors
    /// Returns `Err` if parsing the source fails
    pub fn lex_strict(&mut self) -> Result<Vec<Token<'a>>, CarbideLexerError> {
        let result = self.lex();

        if let Some(first_error) = result.errors.into_iter().next() {
            Err(first_error)
        } else {
            Ok(result.tokens)
        }
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempt to lex a number [`Token`]
    ///
    /// # Errors
    /// Returns `Err` if fails
    fn lex_number(
        &mut self,
        start: u64,
        start_loc: SourceLocation,
    ) -> Result<Token<'a>, CarbideLexerError> {
        if self.src[self.pos..].starts_with("0x") {
            self.pos += 2;
            self.column += 2;

            let hex_start = self.pos;
            self.consume_while(|c| c.is_ascii_hexdigit());

            let end = self.pos as u64;
            let end_loc = self.current_location();
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            let hex_digits = &self.src[hex_start..self.pos];

            if hex_digits.is_empty() {
                return Err(CarbideLexerError::InvalidHexLiteral(
                    "0x".to_string(),
                    start_loc,
                ));
            }

            return Ok(Token {
                token_type: Tokens::HexLiteral(i64::from_str_radix(hex_digits, 16).map_err(
                    |_| CarbideLexerError::InvalidHexLiteral(hex_digits.to_string(), start_loc),
                )?),
                start: start_loc,
                end: end_loc,
                span: start..end,
                src: slice,
            });
        }

        if self.src[self.pos..].starts_with("0b") {
            self.pos += 2;
            self.column += 2;

            let bin_start = self.pos;
            self.consume_while(|c| c == '0' || c == '1');

            let end = self.pos as u64;
            let end_loc = self.current_location();
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            let bin_digits = &self.src[bin_start..self.pos];

            if bin_digits.is_empty() {
                return Err(CarbideLexerError::InvalidBinaryLiteral(
                    "0b".to_string(),
                    start_loc,
                ));
            }

            return Ok(Token {
                token_type: Tokens::BinaryLiteral(i64::from_str_radix(bin_digits, 2).map_err(
                    |_| CarbideLexerError::InvalidBinaryLiteral(bin_digits.to_string(), start_loc),
                )?),
                start: start_loc,
                end: end_loc,
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
        let end_loc = self.current_location();
        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        if has_dot {
            Ok(Token {
                token_type: Tokens::FloatLiteral(slice.parse::<f64>().map_err(|_| {
                    CarbideLexerError::InvalidFloatLiteral(slice.to_string(), start_loc)
                })?),
                start: start_loc,
                end: end_loc,
                span: start..end,
                src: slice,
            })
        } else {
            Ok(Token {
                token_type: Tokens::IntLiteral(slice.parse::<i64>().map_err(|_| {
                    CarbideLexerError::InvalidIntegerLiteral(slice.to_string(), start_loc)
                })?),
                start: start_loc,
                end: end_loc,
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
    fn lex_identifier(
        &mut self,
        start: u64,
        start_loc: SourceLocation,
    ) -> Result<Token<'a>, CarbideLexerError> {
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
        let end = self.pos as u64;
        let end_loc = self.current_location();

        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        let token_type = if let Ok(keyword) = Keywords::try_from(slice) {
            Tokens::Keyword(keyword)
        } else {
            Tokens::Identifier(slice)
        };

        Ok(Token {
            token_type,
            start: start_loc,
            end: end_loc,
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
    fn lex_operator(
        &mut self,
        start: u64,
        start_loc: SourceLocation,
    ) -> Result<Token<'a>, CarbideLexerError> {
        let first_ch = self
            .next()
            .ok_or(CarbideLexerError::UnexpectedEOF(start_loc))?;

        if let Some(second_ch) = self.peek() {
            let two_char_start = self.pos - first_ch.len_utf8();
            let two_char_slice = &self.src[two_char_start..self.pos + second_ch.len_utf8()];

            if BinaryOperators::try_from(two_char_slice).is_ok() {
                self.next();
                let end = self.pos as u64;
                let end_loc = self.current_location();
                let slice = &self.src[usize_from(start)?..usize_from(end)?];

                return Ok(Token {
                    token_type: Tokens::BinaryOperator(BinaryOperators::try_from(slice)?),
                    start: start_loc,
                    end: end_loc,
                    span: start..end,
                    src: slice,
                });
            }

            if UnaryOperators::try_from(two_char_slice).is_ok() {
                self.next();
                let end = self.pos as u64;
                let end_loc = self.current_location();
                let slice = &self.src[usize_from(start)?..usize_from(end)?];

                return Ok(Token {
                    token_type: Tokens::UnaryOperator(UnaryOperators::try_from(slice)?),
                    start: start_loc,
                    end: end_loc,
                    span: start..end,
                    src: slice,
                });
            }
        }

        let end = self.pos as u64;
        let end_loc = self.current_location();
        let slice = &self.src[usize_from(start)?..usize_from(end)?];

        if let Ok(bin_op) = BinaryOperators::try_from(slice) {
            return Ok(Token {
                token_type: Tokens::BinaryOperator(bin_op),
                start: start_loc,
                end: end_loc,
                span: start..end,
                src: slice,
            });
        }

        if let Ok(un_op) = UnaryOperators::try_from(slice) {
            return Ok(Token {
                token_type: Tokens::UnaryOperator(un_op),
                start: start_loc,
                end: end_loc,
                span: start..end,
                src: slice,
            });
        }

        Err(CarbideLexerError::UnexpectedChar(first_ch, start_loc))
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempt to lex a single-character token (`()[]{},;:`)
    ///
    /// # Errors
    /// Returns `Err` if parsing the source fails
    fn lex_single_char(
        &mut self,
        start: u64,
        start_loc: SourceLocation,
    ) -> Result<Option<Token<'a>>, CarbideLexerError> {
        if let Some(ch) = self.peek()
            && Tokens::starts_with(ch)
        {
            self.next();
            let end = self.pos as u64;
            let end_loc = self.current_location();
            let slice = &self.src[usize_from(start)?..usize_from(end)?];

            if let Some(token_type) = Tokens::from_char(ch) {
                return Ok(Some(Token {
                    token_type,
                    start: start_loc,
                    end: end_loc,
                    span: start..end,
                    src: slice,
                }));
            }
        }
        Ok(None)
    }
}

impl<'a> CarbideLexer<'a> {
    /// Attempt to lex a string
    ///
    /// # Errors
    /// Returns `Err` if parsing the source fails
    fn lex_string(
        &mut self,
        start: u64,
        start_loc: SourceLocation,
    ) -> Result<Option<Token<'a>>, CarbideLexerError> {
        if let Some(ch) = self.peek()
            && ch == '"'
        {
            self.next();
            let string_start = self.pos;
            let mut has_interpolation = false;

            loop {
                if self.is_eof() {
                    return Err(CarbideLexerError::UnclosedString(start_loc));
                }

                if let Some(ch) = self.peek() {
                    if ch == '"' {
                        break;
                    } else if ch == '\\' {
                        self.next();
                        if !self.is_eof() {
                            self.next();
                        }
                    } else if ch == '{' {
                        has_interpolation = true;
                        self.next();
                    } else {
                        self.next();
                    }
                }
            }

            let raw_string = &self.src[string_start..self.pos];
            self.next();

            let end = self.pos as u64;
            let end_loc = self.current_location();
            let full_slice = &self.src[usize_from(start)?..usize_from(end)?];

            if has_interpolation {
                let parts = self.lex_interpolated_string(raw_string, start_loc)?;
                return Ok(Some(Token {
                    token_type: Tokens::InterpolatedString(parts),
                    start: start_loc,
                    end: end_loc,
                    span: start..end,
                    src: full_slice,
                }));
            } else {
                let content = self.unescape_string(raw_string)?;
                return Ok(Some(Token {
                    token_type: Tokens::StringLiteral(content),
                    start: start_loc,
                    end: end_loc,
                    span: start..end,
                    src: full_slice,
                }));
            }
        }
        Ok(None)
    }

    /// Attempt to unescape a string
    ///
    /// # Errors
    /// Returns `Err` if lexing the source fails
    fn unescape_string(&self, raw: &str) -> Result<String, CarbideLexerError> {
        let mut result = String::new();
        let mut chars = raw.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        '0' => result.push('\0'),
                        _ => {
                            // TODO: Maybe push a warning about unknown escape sequences
                            result.push('\\');
                            result.push(next_ch);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    /// Attempt to lex an interpolated string
    ///
    /// # Errors
    /// Returns `Err` if lexing the source fails
    fn lex_interpolated_string(
        &self,
        raw: &str,
        loc: SourceLocation,
    ) -> Result<Vec<StringPart>, CarbideLexerError> {
        let mut parts = Vec::new();
        let mut current = 0;
        let bytes = raw.as_bytes();

        while current < bytes.len() {
            let mut text_end = current;
            let mut in_escape = false;

            while text_end < bytes.len() {
                if in_escape {
                    in_escape = false;
                    text_end += 1;
                    continue;
                }

                match bytes[text_end] {
                    b'\\' => {
                        in_escape = true;
                        text_end += 1;
                    }
                    b'{' => break,
                    _ => text_end += 1,
                }
            }

            if text_end > current {
                let text = &raw[current..text_end];
                let unescaped = self.unescape_string(text)?;
                if !unescaped.is_empty() {
                    parts.push(StringPart::Text(unescaped));
                }
            }

            if text_end >= bytes.len() {
                break;
            }

            if bytes[text_end] == b'{' {
                text_end += 1;
                let interp_start = text_end;

                let mut brace_depth = 1;
                while text_end < bytes.len() && brace_depth > 0 {
                    match bytes[text_end] {
                        b'{' => brace_depth += 1,
                        b'}' => brace_depth -= 1,
                        _ => {}
                    }
                    if brace_depth > 0 {
                        text_end += 1;
                    }
                }

                if brace_depth != 0 {
                    return Err(CarbideLexerError::UnmatchedBrace(loc));
                }

                let interp = &raw[interp_start..text_end];
                parts.push(StringPart::Interpolation(interp.to_string()));
                current = text_end + 1;
            }
        }

        Ok(parts)
    }
}
