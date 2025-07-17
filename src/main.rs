use clap::Parser;
use std::{error::Error, fs, path::PathBuf};

use tplayer::{audio::AudioProvider, files::SourceProvider};

/// Terminal music player because GUIs don't like wayland
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source directory
    #[arg(short, long, default_value = "~/tplayer/")]
    source: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Source directory set to `{}`", args.source.display());
    if !args.source.exists() {
        println!("Source Directory doesn't exist, Generating...");
        fs::create_dir_all(args.source.clone()).expect("Failed to generate directory");
    }

    let source = SourceProvider::build(args.source)?;
    let audio = AudioProvider::new();

    for folder in source.playlists {
        println!("- {}", folder.name);
        for song in folder.songs {
            println!("! {}", song.name)
        }
    }

    Ok(())
}
