use std::io::{Result, Write};

use rand::{
    Rng,
    distr::{Alphabetic, Alphanumeric, SampleString},
};

mod action;
mod centered_text;
mod scene;

pub struct GeneratorOptions {
    allow_markdown: bool,
    allow_forced: bool,
}

pub trait Generator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng;
}

pub fn generate_text<R>(
    rng: &mut R,
    len: usize,
    allow_markdown: bool,
    include_numbers: bool,
) -> String
where
    R: Rng,
{
    let res = if include_numbers {
        Alphanumeric.sample_string(rng, len)
    } else {
        Alphabetic.sample_string(rng, len)
    };

    res
}
