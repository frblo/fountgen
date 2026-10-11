use std::io::{Result, Write};

use rand::{Rng, RngExt};

use crate::generators::{Generator, GeneratorOptions, generate_text};

pub struct ActionGenerator {}

impl Generator for ActionGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        let force = opts.allow_forced && rng.random_bool(0.2);
        if force {
            write!(writer, "!")?;
        }

        let action_len = rng.random_range(2..1000);
        write!(
            writer,
            "{}",
            generate_text(rng, action_len / 2, opts.allow_markdown, true)
        )?;

        // Ensures we don't accidently create a character
        if !force {
            write!(
                writer,
                "{}",
                generate_text(rng, action_len / 2, opts.allow_markdown, false)
            )?;
        }

        write!(
            writer,
            "{}",
            generate_text(rng, action_len / 2, opts.allow_markdown, true)
        )?;

        writeln!(writer)?;
        Ok(())
    }
}
