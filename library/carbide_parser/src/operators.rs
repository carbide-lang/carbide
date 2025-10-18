use crate::errors::CarbideParserError;

macro_rules! define_bin_ops {
    ($($kw:ident => $lit:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum BinaryOperators {
            $($kw),*
        }

        impl BinaryOperators {
            pub const ALL: &'static [Self] = &[
                $(Self::$kw),*
            ];

            /// Return the `&str` representation of the Keyword
            #[must_use]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$kw => $lit),*
                }
            }

            /// Check if any [`BinaryOperator`][BinaryOperators] starts with the given char
            #[must_use]
            pub fn starts_with(ch: char) -> bool {
                Self::ALL.iter().any(|op| op.as_str().starts_with(ch))
            }
        }

        impl<'a> TryFrom<&'a str> for BinaryOperators {
            type Error = CarbideParserError;

            fn try_from(ident: &'a str) -> Result<Self, Self::Error> {
                match ident {
                    $($lit => Ok(Self::$kw),)*
                    _ => Err(CarbideParserError::CastBinaryOpFailed(ident.to_string())),
                }
            }
        }
    };
}

macro_rules! define_unary_ops {
    ($($kw:ident => $lit:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum UnaryOperators {
            $($kw),*
        }

        impl UnaryOperators {
            pub const ALL: &'static [Self] = &[
                $(Self::$kw),*
            ];

            #[must_use]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$kw => $lit),*
                }
            }

            /// Check if any [`UnaryOperator`][UnaryOperators] starts with the given char
            #[must_use]
            pub fn starts_with(ch: char) -> bool {
                Self::ALL.iter().any(|op| op.as_str().starts_with(ch))
            }
        }

        impl<'a> TryFrom<&'a str> for UnaryOperators {
            type Error = CarbideParserError;

            fn try_from(ident: &'a str) -> Result<Self, Self::Error> {
                match ident {
                    $($lit => Ok(Self::$kw),)*
                    _ => Err(CarbideParserError::CastUnaryOpFailed(ident.to_string())),
                }
            }
        }
    };
}

define_bin_ops! {
    Eq => "==",
    NotEq  => "!=",
}

define_unary_ops! {
    Not => "!",
}
