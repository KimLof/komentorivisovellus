mod file_types;

use file_types::{
    AUDIO_EXTENSIONS, DOCUMENT_EXTENSIONS, IMAGE_EXTENSIONS, VIDEO_EXTENSIONS, ARCHIVE_EXTENSIONS,
};
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Kysy käyttäjältä lähtöhakemiston polku
    println!("Mikä on polku kansioon, missä haluat siirtää tiedostoja?");
    let mut src_dir: String = String::new();
    io::stdin().read_line(&mut src_dir)?;
    let src_dir: &str = src_dir.trim(); // Poista whitespace, kuten rivinvaihto

    // Kysy, haluaako käyttäjä käydä läpi alihakemistot
    println!("Haluatko käydä alihakemistot myös läpi? [Y/N]");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let recursive: bool = input.trim().eq_ignore_ascii_case("Y");

    // Kysy, mitä tiedostotyyppejä käyttäjä haluaa siirtää
    println!("Mitkä tiedostot haluat siirtää?");
    println!("1: Kuvat");
    println!("2: Äänet");
    println!("3: Videot");
    println!("4: Asiakirjat");
    println!("5: Arkistot");
    println!("6: Kaikki");
    input.clear();
    io::stdin().read_line(&mut input)?;
    let choice: &str = input.trim();

    match choice {
        "1" => move_files(src_dir, "sorted_images", &IMAGE_EXTENSIONS, recursive),
        "2" => move_files(src_dir, "sorted_audio", &AUDIO_EXTENSIONS, recursive),
        "3" => move_files(src_dir, "sorted_videos", &VIDEO_EXTENSIONS, recursive),
        "4" => move_files(src_dir, "sorted_documents", &DOCUMENT_EXTENSIONS, recursive),
        "5" => move_files(src_dir, "sorted_archives", &ARCHIVE_EXTENSIONS, recursive),
        "6" => {
            move_files(src_dir, "sorted_images", &IMAGE_EXTENSIONS, recursive)?;
            move_files(src_dir, "sorted_audio", &AUDIO_EXTENSIONS, recursive)?;
            move_files(src_dir, "sorted_videos", &VIDEO_EXTENSIONS, recursive)?;
            move_files(src_dir, "sorted_documents", &DOCUMENT_EXTENSIONS, recursive)?;
            move_files(src_dir, "sorted_archives", &ARCHIVE_EXTENSIONS, recursive)?;
            Ok(())
        }
        _ => {
            println!("Virheellinen valinta.");
            Ok(())
        }
    }
}

fn move_files(src_dir: &str, dest_suffix: &str, extensions: &[&str], recursive: bool) -> io::Result<()> {
    let mut dest_dir_created: bool = false; // Lipput muistuttaa, onko kohdekansio luotu
    let dest_dir: std::path::PathBuf = Path::new(src_dir).join(dest_suffix);

    let entries: fs::ReadDir = fs::read_dir(src_dir)?;
    for entry in entries {
        let entry: DirEntry = entry?;
        let path: std::path::PathBuf = entry.path();

        if path.is_dir() {
            if recursive {
                move_files(path.to_str().unwrap(), dest_suffix, extensions, recursive)?;
            }
            continue;
        }

        if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
            if extensions.contains(&ext) {
                // Luodaan kohdekansio vasta, kun ensimmäinen sopiva tiedosto löytyy
                if !dest_dir_created {
                    fs::create_dir_all(&dest_dir)?;
                    dest_dir_created = true;
                }
                
                let new_location: std::path::PathBuf = dest_dir.join(entry.file_name());
                fs::rename(&path, &new_location)?;
                println!("Siirretty tiedosto: {:?}", new_location);
            }
        }
    }

    Ok(())
}
