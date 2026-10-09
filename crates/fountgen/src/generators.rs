use std::io::{Result, Write};

mod scene;

pub struct GeneratorOptions {
    markdown_formatting: bool,
    allow_forced: bool,
}

pub trait Generator {
    fn generate(&self, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>;
}
