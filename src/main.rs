use clap::Parser;
use std::{fs, path::PathBuf};

use tplayer::{app::App, audio::AudioHandler, files::SourceHandler};

/// Terminal music player because GUIs don't like wayland
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source directory
    #[arg(short, long, default_value = "~/tplayer/")]
    source: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
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

    // Create directory if needed
    if !fs::exists(absolute_source.clone())? {
        println!("Source Directory doesn't exist, Generating...");
        fs::create_dir_all(absolute_source.clone()).expect("Failed to generate directory");
    }

    // Init Handlers
    let source = SourceHandler::build(absolute_source)?;
    let audio = AudioHandler::new();

    let terminal = ratatui::init();
    let result = App::new(source, audio).run(terminal);
    ratatui::restore();
    result
}
