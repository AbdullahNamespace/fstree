mod config;
mod generator;
mod icons;
use clap::Parser;
use config::TreeConfig;
use generator::Generator;

fn main() -> anyhow::Result<()> {
    let config = TreeConfig::parse();

    let generator = Generator::new(config);
    generator.run()?;

    Ok(())
}
