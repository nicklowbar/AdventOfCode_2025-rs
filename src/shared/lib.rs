use anyhow::{Context, Result};
use clap::Parser;
use displaydoc::Display;
use std::{fs::File, path::PathBuf};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Define a struct to hold the command-line arguments
#[derive(Debug, Parser, Display)]
struct Args {
    solution: u32,
    input_path: PathBuf,
}

fn init_tracing() {
    // Only run once; protects against multiple initialization attempts
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env()) // respects RUST_LOG
            .with(fmt::layer()) // pretty logging
            .init(); // installs as global default
    });
}

type SolutionFunction = fn(&File) -> Result<u64>;

pub fn shared_main(solution1: SolutionFunction, solution2: SolutionFunction) -> Result<()> {
    init_tracing();
    let args = Args::parse();
    info!("Input arguments: {:?}", args);

    let input = File::open(args.input_path).with_context(|| "Unable to open input file")?;
    let value = match args.solution {
        1 => solution1(&input).with_context(|| "Exception encountered with executing solution")?,
        2 => solution2(&input).with_context(|| "Exception encountered with executing solution")?,
        default => panic!("Invalid solution index: {default}"),
    };
    info!("Solution: {value}");

    Ok(())
}
