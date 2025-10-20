use crate::codes::ErrCode;

pub trait CarbideError {
    /// The [`ariadne::Span`] type for this error
    type Span: ariadne::Span;

    /// Get the [`ErrCode`] associated with this `CarbideError`
    fn code(&self) -> ErrCode;

    /// Get the error message associated with this `CarbideError`
    fn message(&self) -> String;

    /// Build a formatted [`ariadne::Report`]
    ///
    /// # Errors
    /// Returns `Err` if building the error fails
    fn report(&'_ self, file: &str, src: &str) -> Result<ariadne::Report<'_, Self::Span>, Self>
    where
        Self: Sized;

    /// Return hints or solutions
    fn help(&self) -> Option<&'static str> {
        None
    }
}
