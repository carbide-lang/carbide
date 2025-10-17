use crate::errors::CarbideParserError;

macro_rules! define_keywords {
    ($($kw:ident => $lit:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Keywords {
            $($kw),*
        }

        impl<'a> TryFrom<&'a str> for Keywords {
            type Error = CarbideParserError;
            fn try_from(ident: &'a str) -> Result<Self, Self::Error> {
                match ident {
                    $($lit => Ok(Keywords::$kw),)*
                    _ => Err(CarbideParserError::CastKeywordFailed(ident.to_string())),
                }
            }
        }
    };
}

define_keywords!(
    Let => "let",
    Fn => "fn"
);