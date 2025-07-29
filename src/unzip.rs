/*
 * Handles unzipping files if needed
 */

use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Context, Error};
use zip::ZipArchive;

/// Convenience function to unzip files so I don't have to manually unzip new albums
pub fn ask_unzip(source: &Path) -> color_eyre::Result<()> {
    // Try to find zip files
    let zip_files: Vec<_> = fs::read_dir(source)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if path.is_file() && path.extension()? == "zip" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // No zip files
    if zip_files.len() == 0 {
        return Ok(());
    };

    // Ask to Cancel
    print!("Zip Files Found In Source\nWould you like to unzip them [Y/n]: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    if buffer.to_lowercase().contains("n") {
        return Ok(());
    }

    // Unzip if not cancelled
    for file in zip_files {
        unzip(file.clone())
            .with_context(|| format!("{}", file.display()))
            .unwrap();
        println!("Extracted {}", file.file_name().unwrap().display());

        // Cleanup
        fs::remove_file(file).expect("Failed to remove used zip file");
    }

    Ok(())
}

pub fn unzip(path: PathBuf) -> Result<(), Error> {
    let file = File::open(path.clone()).expect("Failed to open zip file");
    let mut archive = ZipArchive::new(file)?;

    // Extract to a directory named after the zip file (without extension)
    let extract_path = path
        .parent()
        .unwrap()
        .join(path.file_stem().unwrap().to_str().unwrap());
    std::fs::create_dir_all(&extract_path)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(&extract_path).join(file.mangled_name());

        if file.name().ends_with('/') {
            // It's a directory
            std::fs::create_dir_all(&outpath)?;
        } else {
            // It's a file
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Set permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}
