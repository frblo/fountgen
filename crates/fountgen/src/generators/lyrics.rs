use std::io::{Result, Write};

use rand::{Rng, RngExt};

use crate::{
    generators::{Generator, GeneratorOptions},
    text::generate_text,
};

pub struct LyricsGenerator {}

impl Generator for LyricsGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        write!(writer, "~")?;

        let len = rng.random_range(2..1000);
        write!(
            writer,
            "{}",
            generate_text(rng, len, opts.allow_markdown, true, true)
        )?;

        writeln!(writer)?;
        Ok(())
    }
}
