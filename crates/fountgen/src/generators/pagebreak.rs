use std::io::{Result, Write};

use rand::Rng;

use crate::generators::{Generator, GeneratorOptions};

pub struct PagebreakGenerator {}

impl Generator for PagebreakGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        write!(writer, "===")?;
        writeln!(writer)?;
        Ok(())
    }
}
