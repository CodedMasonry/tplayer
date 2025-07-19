use clap::Parser;
use std::{error::Error, fs, path::PathBuf};

use tplayer::{audio::AudioProvider, files::SourceProvider};

/// Terminal music player because GUIs don't like wayland
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source directory
    #[arg(short, long, default_value = "~/tplayer/")]
    source: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // I don't wanna write out the whole home directory, so fill it in
    let absolute_source = PathBuf::from(
        args.source.replace(
            '~',
            std::env::home_dir()
                .expect("You somehow don't have a home")
                .to_str()
                .unwrap(),
        ),
    );
    println!("Source directory set to `{}`", absolute_source.display());

    if !fs::exists(absolute_source.clone())? {
        println!("Source Directory doesn't exist, Generating...");
        fs::create_dir_all(absolute_source.clone()).expect("Failed to generate directory");
    }

    let source = SourceProvider::build(absolute_source)?;
    let audio = AudioProvider::new();

    for folder in source.playlists {
        println!("- {}", folder.name);
        for song in folder.songs {
            println!("! {}", song.name)
        }
    }

    Ok(())
}
