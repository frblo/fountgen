use std::io::{Result, Write};

use rand::{Rng, RngExt, seq::IndexedRandom};

use crate::generators::{Generator, GeneratorOptions, generate_text};

pub struct SceneGenerator {}

impl Generator for SceneGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        let force = opts.allow_forced && rng.random_bool(0.5);
        if force {
            write!(writer, ".")?;
        } else {
            let prefix = *VALID_SCENE_PREFIXES.choose(rng).unwrap();
            write!(writer, "{}. ", prefix)?;
        }

        let heading_len = rng.random_range(1..100);
        write!(
            writer,
            "{}",
            generate_text(rng, heading_len, opts.allow_markdown, true)
        )?;

        // Write scene number?
        if rng.random_bool(0.2) {
            write!(writer, " #")?;
            let number_len = rng.random_range(1..6);
            write!(writer, "{}", generate_text(rng, number_len, false, true))?;
            write!(writer, "#")?;
        }

        writeln!(writer)?;
        Ok(())
    }
}

const VALID_SCENE_PREFIXES: &[&str] = &["INT", "EXT", "EST", "INT./EXT", "INT/EXT", "I/E"];
