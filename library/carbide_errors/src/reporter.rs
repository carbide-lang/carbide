use crate::error::CarbideError;
use ariadne::{Cache, Source};
use std::collections::HashMap;

pub struct ErrorReporter {
    sources: HashMap<String, String>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, filename: impl Into<String>, content: impl Into<String>) {
        self.sources.insert(filename.into(), content.into());
    }

    pub fn print_error<E>(&self, filename: &str, error: &E)
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        let src = self.sources.get(filename).map(|s| s.as_str()).unwrap_or("");

        let report = error.report(filename, src);

        let mut cache = SimpleCache::new();
        cache.insert(filename.to_string(), src.to_string());

        report
            .eprint(&mut cache)
            .unwrap_or_else(|e| eprintln!("Failed to print error report: {}", e));
    }

    pub fn print_errors<E>(&self, filename: &str, errors: &[E])
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        for error in errors {
            self.print_error(filename, error);
        }
    }

    pub fn format_error<E>(&self, filename: &str, error: &E) -> String
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        let src = self.sources.get(filename).map(|s| s.as_str()).unwrap_or("");

        let report = error.report(filename, src);

        let mut cache = SimpleCache::new();
        cache.insert(filename.to_string(), src.to_string());

        let mut buffer = Vec::new();
        report
            .write(&mut cache, &mut buffer)
            .unwrap_or_else(|e| eprintln!("Failed to format error: {}", e));

        String::from_utf8_lossy(&buffer).to_string()
    }

    pub fn format_errors<E>(&self, filename: &str, errors: &[E]) -> String
    where
        E: CarbideError,
        E::Span: ariadne::Span<SourceId = String>,
    {
        errors
            .iter()
            .map(|e| self.format_error(filename, e))
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
        self.sources.get(id).ok_or_else(|| {
            Box::new(format!("Source '{}' not found", id)) as Box<dyn std::fmt::Debug>
        })
    }

    fn display<'a>(&self, id: &'a String) -> Option<impl std::fmt::Display + 'a> {
        Some(Box::new(id.clone()))
    }
}
