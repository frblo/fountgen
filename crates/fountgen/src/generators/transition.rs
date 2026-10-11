use std::io::{Result, Write};

use rand::{Rng, RngExt};

use crate::{
    generators::{Generator, GeneratorOptions},
    text::generate_text,
};

pub struct TransitionGenerator {}

impl Generator for TransitionGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        let len = rng.random_range(1..100);
        let force = opts.allow_forced && rng.random_bool(0.5);
        if force {
            write!(writer, ">")?;
            write!(
                writer,
                "{}",
                generate_text(rng, len, opts.allow_markdown, true, true)
            )?;
        } else {
            writeln!(writer)?;
            write!(
                writer,
                "{} TO:",
                generate_text(rng, len, opts.allow_markdown, true, true).to_uppercase()
            )?;
        }

        writeln!(writer)?;
        Ok(())
    }
}
