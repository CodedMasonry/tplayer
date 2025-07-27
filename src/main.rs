use clap::Parser;
use std::{fs, path::PathBuf};

use tplayer::{app::App, audio::AudioHandler, config::Config, files::SourceHandler, unzip};

/// Terminal music player because GUIs don't like wayland
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source directory
    #[arg(short, long, default_value = "~/Music/")]
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

    // Handle unzip files in source if any
    unzip::ask_unzip(&absolute_source)?;

    // Init Handlers
    let source =
        SourceHandler::build(absolute_source.clone()).map_err(|e| color_eyre::eyre::eyre!(e))?;
    let audio = AudioHandler::new();

    // Init & Handle Config
    let config = Config::parse_or_new(&absolute_source.join("tplayer_config.json"));
    audio.sink.set_volume(config.volume);

    // Run UI
    let terminal = ratatui::init();
    let result = App::new(source, audio, config).run(terminal);
    ratatui::restore();
    result
}
