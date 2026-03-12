//! TypeScript parser placeholder
use crate::error::Result;
use crate::types::Symbol;

pub struct TypeScriptParser;

impl TypeScriptParser {
    pub fn new() -> Self {
        Self
    }
    pub fn parse(&self, _source: &str, _file_path: &str) -> Result<Vec<Symbol>> {
        Ok(vec![])
    }
}
