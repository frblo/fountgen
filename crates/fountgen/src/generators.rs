use std::io::{Result, Write};

use rand::{
    Rng, RngExt,
    seq::{IndexedRandom, index::sample},
};

use crate::generators::{
    FountainElements::{
        Action, CenteredText, Dialouge, DualDialogue, Lyrics, Pagebreak, Transition,
    },
    action::ActionGenerator,
    centered_text::CenteredTextGenerator,
    dialouge::{DialogueGenerator, DualDialogueGenerator},
    lyrics::LyricsGenerator,
    pagebreak::PagebreakGenerator,
    scene::SceneGenerator,
    transition::TransitionGenerator,
};

mod action;
mod centered_text;
mod dialouge;
mod lyrics;
mod pagebreak;
mod scene;
mod titlepage;
mod transition;

pub struct GeneratorOptions {
    /// Allow markdown formatting of content, like `*text*`.
    pub allow_markdown: bool,
    /// Allow elements to be forced.
    pub allow_forced: bool,
    /// The number of screenplay elements which are allowed.
    pub element_count: Option<usize>,
    /// The number of scenes which are allowed.
    pub scene_count: Option<usize>,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        Self {
            allow_markdown: true,
            allow_forced: true,
            element_count: None,
            scene_count: None,
        }
    }
}

pub trait Generator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng;
}

pub struct FountainGenerator {}

impl Generator for FountainGenerator {
    fn generate<R>(rng: &mut R, opts: &GeneratorOptions, writer: &mut dyn Write) -> Result<()>
    where
        R: Rng,
    {
        let scene_count = opts.scene_count.unwrap_or(rng.random_range(1..50));
        let element_count = opts
            .element_count
            .unwrap_or(rng.random_range(scene_count..(scene_count * 10)))
            - scene_count;

        let elements_in_scenes = divide_elements(rng, element_count, scene_count);

        for elements_in_scene in elements_in_scenes {
            SceneGenerator::generate(rng, opts, writer)?;
            writeln!(writer)?;
            for _ in 0..elements_in_scene {
                let element = &ELEMENT_WEIGHTS.choose_weighted(rng, |e| e.1).unwrap().0;
                match element {
                    Action => ActionGenerator::generate(rng, opts, writer),
                    Dialouge => DialogueGenerator::generate(rng, opts, writer),
                    DualDialogue => DualDialogueGenerator::generate(rng, opts, writer),
                    Lyrics => LyricsGenerator::generate(rng, opts, writer),
                    Transition => TransitionGenerator::generate(rng, opts, writer),
                    CenteredText => CenteredTextGenerator::generate(rng, opts, writer),
                    Pagebreak => PagebreakGenerator::generate(rng, opts, writer),
                }?;
                writeln!(writer)?;
            }
        }

        Ok(())
    }
}

pub enum FountainElements {
    Action,
    Dialouge,
    DualDialogue,
    Lyrics,
    Transition,
    CenteredText,
    Pagebreak,
}

const ELEMENT_WEIGHTS: &[(FountainElements, usize)] = &[
    (Action, 10),
    (Dialouge, 10),
    (DualDialogue, 2),
    (Lyrics, 2),
    (Transition, 3),
    (CenteredText, 4),
    (Pagebreak, 1),
];

fn divide_elements<R>(rng: &mut R, element_count: usize, scene_count: usize) -> Vec<usize>
where
    R: Rng,
{
    let sample = sample(rng, element_count + scene_count - 1, scene_count - 1);
    let mut cuts: Vec<usize> = sample.into_iter().map(|i| i + 1).collect();
    cuts.sort_unstable();

    let mut all_cuts = Vec::with_capacity(scene_count + 1);
    all_cuts.push(0);
    all_cuts.extend(cuts);
    all_cuts.push(element_count + scene_count);

    let mut result = Vec::with_capacity(scene_count);
    for i in 0..scene_count {
        result.push(all_cuts[i + 1] - all_cuts[i] - 1);
    }

    result
}
