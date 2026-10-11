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

    /// The number of elements (including scenes) in the script
    #[arg(short = 'e', long = "element-count")]
    element_count: Option<usize>,

    /// Allow markdown style formatting in output
    #[arg(long = "markdown", default_value_t = true)]
    allow_markdown: bool,

    /// Allow forced elements
    #[arg(long = "forced", default_value_t = true)]
    allow_forced: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut rng = rand::rng();

    let mut writer = BufWriter::new(stdout());

    let opts = cli.into();

    FountainGenerator::generate(&mut rng, &opts, &mut writer)?;

    Ok(())
}

impl From<Cli> for GeneratorOptions {
    fn from(cli: Cli) -> Self {
        GeneratorOptions {
            allow_markdown: cli.allow_markdown,
            allow_forced: cli.allow_forced,
            element_count: cli.scene_count,
            scene_count: cli.element_count,
        }
    }
}
