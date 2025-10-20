use crate::error::CarbideError;
use ariadne::{Cache, Source};
use std::collections::HashMap;

pub struct ErrorReporter {
    sources: HashMap<String, String>,
}

impl ErrorReporter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, filename: impl Into<String>, content: impl Into<String>) {
        self.sources.insert(filename.into(), content.into());
    }

    /// Print a [`CarbideError`] given a `filename`
    ///
    /// # Errors
    /// Returns `Err` if printing the errors fail, or if [`CarbideError::report()`] fails
    pub fn print_error<E>(&self, filename: &str, error: &Box<E>) -> Result<(), String>
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        let src = self.sources.get(filename).map_or("", |s| s.as_str());

        let report = error.report(filename, src);

        let mut cache = SimpleCache::new();
        cache.insert(filename.to_string(), src.to_string());

        report
            .map_err(|_| "Failed to get report")?
            .eprint(&mut cache)
            .map_err(|e| format!("Failed to print error: {e}"))
    }

    /// Print all errors associated with this `ErrorReporter` given a `filename`
    ///
    /// # Errors
    /// Returns `Err` if printing the errors fail
    pub fn print_errors<E>(&self, filename: &str, errors: &[Box<E>]) -> Result<(), String>
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        for error in errors {
            self.print_error(filename, error)?;
        }

        Ok(())
    }

    /// Get a formatted [`CarbideError`]
    ///
    /// # Errors
    /// Returns `Err` if getting the report fails
    pub fn format_error<E>(&self, filename: &str, error: &Box<E>) -> Result<String, String>
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        let src = self.sources.get(filename).map_or("", |s| s.as_str());

        let report = error.report(filename, src);

        let mut cache = SimpleCache::new();
        cache.insert(filename.to_string(), src.to_string());

        let mut buffer = Vec::new();
        report
            .map_err(|_| "Failed to get report")?
            .write(&mut cache, &mut buffer)
            .unwrap_or_else(|e| eprintln!("Failed to format error: {e}"));

        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    pub fn format_errors<E>(&self, filename: &str, errors: &[Box<E>]) -> String
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        errors
            .iter()
            .filter_map(|e| self.format_error(filename, e).ok())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

struct SimpleCache {
    sources: HashMap<String, Source>,
}

impl SimpleCache {
    fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    fn insert(&mut self, id: String, source: String) {
        self.sources.insert(id, Source::from(source));
    }
}

impl ariadne::Cache<String> for SimpleCache {
    type Storage = String;

    fn fetch(
        &mut self,
        id: &String,
    ) -> Result<&Source<<Self as Cache<String>>::Storage>, impl std::fmt::Debug> {
        self.sources
            .get(id)
            .ok_or_else(|| Box::new(format!("Source '{id}' not found")) as Box<dyn std::fmt::Debug>)
    }

    fn display<'a>(&self, id: &'a String) -> Option<impl std::fmt::Display + 'a> {
        Some(Box::new(id.clone()))
    }
}
