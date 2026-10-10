use std::io::{Result, Write};

use rand::{Rng, RngExt};

use crate::generators::{Generator, GeneratorOptions, generate_text};

pub struct CenteredTextGenerator {}

impl Generator for CenteredTextGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        write!(writer, ">")?;

        let centered_text_len = rng.random_range(2..1000);
        write!(
            writer,
            "{}",
            generate_text(rng, centered_text_len, opts.allow_markdown, true)
        )?;

        write!(writer, "<")?;
        writeln!(writer)?;
        Ok(())
    }
}
