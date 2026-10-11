use std::io::{Result, Write};

use rand::{Rng, RngExt, seq::IndexedRandom};

use crate::{
    generators::{Generator, GeneratorOptions},
    text::generate_text,
};

pub struct DialogueGenerator {}

impl Generator for DialogueGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        generate_dialouge(rng, opts, writer, false)
    }
}

pub struct DualDialogueGenerator {}

impl Generator for DualDialogueGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        generate_dialouge(rng, opts, writer, false)?;
        generate_dialouge(rng, opts, writer, true)
    }
}

fn generate_dialouge<R>(
    rng: &mut R,
    opts: &GeneratorOptions,
    writer: &mut dyn Write,
    dual: bool,
) -> Result<()>
where
    R: Rng,
{
    let force = opts.allow_forced && rng.random_bool(0.2);
    let name_len = rng.random_range(1..10);
    if force {
        write!(
            writer,
            "@{}",
            generate_text(rng, name_len, opts.allow_markdown, true, false)
        )?;
    } else {
        write!(
            writer,
            "{}",
            generate_text(rng, name_len, opts.allow_markdown, false, false).to_uppercase()
        )?;
    }
    if dual {
        write!(writer, "^",)?;
    }

    writeln!(writer)?;

    let element_count = rng.random_range(1..10);
    for _ in 0..element_count {
        match ELEMENTS.choose(rng).unwrap() {
            Element::Line => LineGenerator::generate(rng, opts, writer),
            Element::Parenthetical => ParentheticalGenerator::generate(rng, opts, writer),
        }?;
    }

    Ok(())
}

struct LineGenerator {}
impl Generator for LineGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        let line_len = rng.random_range(1..100);
        writeln!(
            writer,
            "{}",
            generate_text(rng, line_len, opts.allow_markdown, true, true)
        )?;
        Ok(())
    }
}

struct ParentheticalGenerator {}
impl Generator for ParentheticalGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        let parenthetical_len = rng.random_range(1..50);
        writeln!(
            writer,
            "({})",
            generate_text(rng, parenthetical_len, opts.allow_markdown, true, true)
        )?;
        Ok(())
    }
}

const ELEMENTS: [Element; 2] = [Element::Line, Element::Parenthetical];

enum Element {
    Line,
    Parenthetical,
}
