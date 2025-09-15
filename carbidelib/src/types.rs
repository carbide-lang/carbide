use crate::errors::ParserError;

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Int,
    Float,
    String,
    Bool,
    Literal(String), // User-defined type
}

impl TryFrom<String> for Types {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        return match value.as_str() {
            "int" => Ok(Types::Int),
            "float" => Ok(Types::Float),
            "string" => Ok(Types::String),
            "bool" => Ok(Types::Bool),
            _ => Ok(Types::Literal(value)),
        };
    }

    type Error = ParserError;
}
