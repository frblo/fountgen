use std::io::{Result, Write};

use rand::Rng;

use crate::generators::{Generator, GeneratorOptions};

pub struct TitlePageGenerator {}

impl Generator for TitlePageGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        todo!()
    }
}
