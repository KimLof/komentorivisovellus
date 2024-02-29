use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::ffi::OsStr;

mod file_types;

use file_types::{
    AUDIO_EXTENSIONS, DOCUMENT_EXTENSIONS, IMAGE_EXTENSIONS, VIDEO_EXTENSIONS, ARCHIVE_EXTENSIONS,
};

// Tiedostojen siirtämiseen tarvittava funktio
fn move_files(src_dir: &str, dest_dir: &str, extensions: &[&str], recursive: bool, depth: usize, preview: bool) -> io::Result<Vec<PathBuf>> {
    const MAX_DEPTH: usize = 15;
    if depth > MAX_DEPTH {
        return Err(io::Error::new(io::ErrorKind::Other, "Rekursion maksimisyvyys saavutettu"));
    }
    let src_dir_full: PathBuf = std::fs::canonicalize(src_dir)?;
    let entries: fs::ReadDir = std::fs::read_dir(&src_dir_full)?;

    // Luo kohdekansio, jos se ei ole olemassa
    std::fs::create_dir_all(dest_dir)?;

    println!("Kohdekansio: {:?}", dest_dir);

    let mut found_files = Vec::new();

    for entry in entries {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();

        if path.is_dir() {
            if recursive {
                // Luo alikansio kohdekansioon, jos se ei ole olemassa
                let subdir = path.file_name().unwrap().to_string_lossy();
                let dest_subdir: PathBuf = Path::new(dest_dir).join(subdir.as_ref());
                fs::create_dir_all(&dest_subdir)?;
            
                // Tarkista, onko alikansiossa siirrettäviä tiedostoja ennen rekursion aloittamista
                let has_movable_files = path.read_dir()?.any(|entry| {
                    if let Ok(entry) = entry {
                        if let Some(ext) = entry.path().extension().and_then(std::ffi::OsStr::to_str) {
                            extensions.contains(&ext)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
            
                if has_movable_files {
                    // Jos esikatselu on käytössä, lisää löydetyt tiedostot listaan
                    if preview {
                        for entry in path.read_dir()? {
                            if let Ok(entry) = entry {
                                if let Some(ext) = entry.path().extension().and_then(std::ffi::OsStr::to_str) {
                                    if extensions.contains(&ext) {
                                        found_files.push(entry.path());
                                    }
                                }
                            }
                        }
                    }
                    
                    // Jos alikansiossa on siirrettäviä tiedostoja, jatka rekursiota
                    if let Err(err) = move_files(&path.to_string_lossy(), &dest_subdir.to_string_lossy(), extensions, recursive, depth + 1, preview) {
                        println!("Virhe käsittelyssä alikansiota: {:?}", err);
                    }
                }
            }
            continue;
        }

        if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
            if extensions.contains(&ext) {
                let dest_file_path = Path::new(dest_dir).join(entry.file_name());
                fs::rename(&path, &dest_file_path)?;
                println!("Siirretty tiedosto: {:?}", dest_file_path);
                found_files.push(dest_file_path);
            }
        }
    }

    Ok(found_files)
}

// GUI-koodi
use iced::{
    button, scrollable, Button, Column, Container, Element, Sandbox, Scrollable, Text, TextInput, pick_list, PickList, Checkbox, Length, Alignment, Space,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileType {
    Image,
    Audio,
    Video,
    Document,
    Archive,
    All,
}

impl FileType {
    const ALL: [FileType; 6] = [
        FileType::Image,
        FileType::Audio,
        FileType::Video,
        FileType::Document,
        FileType::Archive,
        FileType::All,
    ];
}

impl Default for FileType {
    fn default() -> FileType {
        FileType::Image
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileType::Image => "Kuvat",
                FileType::Audio => "Äänet",
                FileType::Video => "Videot",
                FileType::Document => "Asiakirjat",
                FileType::Archive => "Arkistot",
                FileType::All => "Kaikki",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SubmitPressed,
    FileTypeSelected(FileType),
    RecursiveToggleChanged(bool),
    PreviewToggleChanged(bool),
}

struct MyGUI {
    input_value: String,
    input_state: iced::text_input::State,
    submit_button_state: iced::button::State,
    output_files: Vec<PathBuf>, // Lisää tämä, jotta voidaan näyttää löydetyt tiedostot GUI:ssa
    file_type: FileType,
    recursive: bool,
    preview: bool, 
    file_type_state: iced::pick_list::State<FileType>,
    scroll: scrollable::State, // Lisää tämä vieritystilaa varten

}

impl Sandbox for MyGUI {
    type Message = Message;

    fn new() -> Self {
        MyGUI {
            input_value: String::new(),
            input_state: iced::text_input::State::new(),
            submit_button_state: iced::button::State::new(),
            output_files: Vec::new(),
            file_type: FileType::default(),
            recursive: false,
            preview: false, // Alustetaan esikatselun asetus pois päältä
            file_type_state: iced::pick_list::State::default(),
            scroll: scrollable::State::new(), // Alusta vieritystila
        }
    }

    fn title(&self) -> String {
        "Tiedostojen Siirto GUI".into()
    }

    

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(input) => {
                self.input_value = input;
            }
            Message::SubmitPressed => {
                let source_dir = Path::new(&self.input_value);
                let source_dir_full = match source_dir.canonicalize() {
                    Ok(path) => path,
                    Err(err) => {
                        self.output_files.clear(); // Tyhjennä löydettyjen tiedostojen lista
                        println!("Error: {}", err);
                        return;
                    }
                };
    
                let extensions: Vec<&str> = match self.file_type {
                    FileType::Image => IMAGE_EXTENSIONS.iter().copied().collect(),
                    FileType::Audio => AUDIO_EXTENSIONS.iter().copied().collect(),
                    FileType::Video => VIDEO_EXTENSIONS.iter().copied().collect(),
                    FileType::Document => DOCUMENT_EXTENSIONS.iter().copied().collect(),
                    FileType::Archive => ARCHIVE_EXTENSIONS.iter().copied().collect(),
                    FileType::All => {
                        let mut all_extensions = Vec::new();
                        all_extensions.extend_from_slice(IMAGE_EXTENSIONS);
                        all_extensions.extend_from_slice(AUDIO_EXTENSIONS);
                        all_extensions.extend_from_slice(VIDEO_EXTENSIONS);
                        all_extensions.extend_from_slice(DOCUMENT_EXTENSIONS);
                        all_extensions.extend_from_slice(ARCHIVE_EXTENSIONS);
                        all_extensions
                    }
                };
    
                // Tee esikatselu
                let mut found_files = Vec::new();
                for &ext in &extensions { // Lisätään tässä desctructuring, jotta saadaan &str
                    let mut walker = WalkDir::new(&source_dir_full);
                    
                    walker
                        .into_iter()
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| {
                            entry
                                .path()
                                .extension()
                                .map_or(false, |e| e == ext) // Vertaa &str:ää OsStr:ään
                        })
                        .for_each(|entry| {
                            println!("Found file: {:?}", entry.path());
                            found_files.push(entry.path().to_path_buf()); // Muuta Path -> PathBuf
                        });
                }
                
                self.output_files = found_files; // Päivitä löydetyt tiedostot
            }
            Message::FileTypeSelected(file_type) => {
                self.file_type = file_type;
            }
            Message::RecursiveToggleChanged(recursive) => {
                self.recursive = recursive;
            }
            Message::PreviewToggleChanged(preview) => {
                self.preview = preview;
            }
        }
    }
    
    fn view(&mut self) -> Element<Message> {
        let file_type_pick_list = PickList::new(
            &mut self.file_type_state,
            &FileType::ALL[..],
            Some(self.file_type),
            Message::FileTypeSelected,
        );
    
        let input = TextInput::new(
            &mut self.input_state,
            "Syötä hakemiston polku...",
            &self.input_value,
            Message::InputChanged,
        );
    
        let button = Button::new(&mut self.submit_button_state, Text::new("Siirrä Tiedostot"))
            .on_press(Message::SubmitPressed);
    
        let recursive_checkbox = Checkbox::new(
            self.recursive,
            "Käsittele alihakemistot",
            Message::RecursiveToggleChanged,
        );
    
        let preview_checkbox = Checkbox::new(
            self.preview,
            "Näytä esikatselu",
            Message::PreviewToggleChanged,
        );
    
        let mut content = Column::new()
            .spacing(10) // Lisää tilaa elementtien välille
            .align_items(Alignment::Center) // Keskittää elementit
            .push(Text::new("Valitse tiedostotyyppi:"))
            .push(file_type_pick_list)
            .push(recursive_checkbox)
            .push(preview_checkbox)
            .push(input)
            .push(button);
    
        // Lisätään vieritettävä alue löydetyille tiedostoille
        if !self.output_files.is_empty() {
            let files_list: Element<_> = self.output_files.iter().fold(Column::new().spacing(5), |column, file| {
                column.push(Text::new(format!("Found file: {:?}", file)))
            }).into();
    
            let scrollable_files_list = Scrollable::new(&mut self.scroll)
                .width(Length::Fill)
                .height(Length::Units(150)) // Aseta korkeus tarpeesi mukaan
                .push(Container::new(files_list).width(Length::Fill).center_x());
    
            content = content.push(Space::with_height(Length::Units(15))); // Lisää tyhjää tilaa
            content = content.push(scrollable_files_list); // Lisää vieritettävä lista löydetyistä tiedostoista
        }
    
        content.into()
    }
    
    
}

fn main() {
    MyGUI::run(Default::default());
}
