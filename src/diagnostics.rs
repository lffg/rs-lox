#[derive(Debug)]
pub struct Diagnostic {
    line: usize,
    message: String,
}

impl Diagnostic {
    pub fn report(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.message);
    }
}

#[derive(Debug, Default)]
pub struct Diagnostics {
    diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Creates a new diagnostic bag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new diagnostic.
    pub fn diagnose(&mut self, line: usize, message: impl Into<String>) {
        let message = message.into();
        self.diagnostics.push(Diagnostic { line, message });
    }

    /// Checks if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Reports all diagnostics.
    pub fn report_all(&self) {
        for diagnostic in &self.diagnostics {
            diagnostic.report();
        }
    }
}
