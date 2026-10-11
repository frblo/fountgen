use std::io::{BufWriter, stdout};

use clap::Parser;
use color_eyre::Result;
use fountgen::{FountainGenerator, Generator, GeneratorOptions};

#[derive(Debug, Parser)]
#[command(name = "fountgen", version, about = "A Fountain script generator")]
struct Cli {
    /// Output path (use '-' for stdout)
    #[arg(short = 'o', long = "out")]
    out: Option<String>,

    /// Alias for stdout (same as `-o -`)
    #[arg(long = "stdout")]
    stdout: bool,

    /// The number of scenes in the script
    #[arg(short = 's', long = "scene-count")]
    scene_count: Option<usize>,

    /// The number of scenes in the script
    #[arg(short = 'e', long = "element-count")]
    element_count: Option<usize>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut rng = rand::rng();

    let mut writer = BufWriter::new(stdout());

    let opts = generator_options(&cli);

    FountainGenerator::generate(&mut rng, &opts, &mut writer)?;

    Ok(())
}

fn generator_options(cli: &Cli) -> GeneratorOptions {
    GeneratorOptions {
        allow_markdown: true,
        allow_forced: true,
        element_count: cli.scene_count,
        scene_count: cli.element_count,
    }
}
