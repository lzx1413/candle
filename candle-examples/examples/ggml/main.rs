use anyhow::Result;
use clap::Parser;
use std::fs::File;

use candle::quantized::ggml_file::Content;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GGML file to load, typically a .bin file generated by the quantize command from llama.cpp
    #[arg(long)]
    model: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.model)?;
    let start = std::time::Instant::now();
    let model = Content::read(&mut file)?;

    println!(
        "Loaded {:?} tensors in {:?}",
        model.tensors.len(),
        start.elapsed()
    );
    Ok(())
}
