use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

mod file_types;

use file_types::{
    AUDIO_EXTENSIONS, DOCUMENT_EXTENSIONS, IMAGE_EXTENSIONS, VIDEO_EXTENSIONS, ARCHIVE_EXTENSIONS,
};

// Tiedostojen siirtämiseen tarvittava funktio
fn move_files(src_dir: &str, dest_dir: &str, extensions: &[&str], recursive: bool) -> io::Result<()> {
    let src_dir: PathBuf = std::fs::canonicalize(src_dir)?;
    let entries: fs::ReadDir = std::fs::read_dir(&src_dir)?;

    // Luo kohdekansio, jos se ei ole olemassa
    std::fs::create_dir_all(dest_dir)?;

    println!("Kohdekansio: {:?}", dest_dir);

    for entry in entries {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();

        if path.is_dir() {
            if recursive {
                // Luo alikansio kohdekansioon, jos se ei ole olemassa
                let subdir = path.file_name().unwrap().to_string_lossy();
                let dest_subdir: PathBuf = Path::new(dest_dir).join(subdir.as_ref());
                fs::create_dir_all(&dest_subdir)?;
                println!("Alikansio: {:?}", dest_subdir);
                move_files(&path.to_string_lossy(), &dest_subdir.to_string_lossy(), extensions, recursive)?;
            }
            continue;
        }

        if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
            if extensions.contains(&ext) {
                let dest_file_path = Path::new(dest_dir).join(entry.file_name());
                fs::rename(&path, &dest_file_path)?;
                println!("Siirretty tiedosto: {:?}", dest_file_path);
            }
        }
    }

    Ok(())
}





use iced::{
    button, Button, Checkbox, Column, Element, Sandbox, Text, TextInput, pick_list, PickList,
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
}

struct MyGUI {
    input_value: String,
    input_state: iced::text_input::State,
    submit_button_state: iced::button::State,
    output_message: String,
    file_type: FileType,
    recursive: bool,
    file_type_state: iced::pick_list::State<FileType>,
}

impl Sandbox for MyGUI {
    type Message = Message;

    fn new() -> Self {
        MyGUI {
            input_value: String::new(),
            input_state: iced::text_input::State::new(),
            submit_button_state: iced::button::State::new(),
            output_message: String::new(),
            file_type: FileType::default(),
            recursive: false,
            file_type_state: iced::pick_list::State::default(),
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
                        self.output_message = format!("Error: {}", err);
                        return;
                    }
                };
    
                match self.file_type {
                    FileType::All => {
                        let extensions = [
                            IMAGE_EXTENSIONS,
                            AUDIO_EXTENSIONS,
                            VIDEO_EXTENSIONS,
                            DOCUMENT_EXTENSIONS,
                            ARCHIVE_EXTENSIONS,
                        ];
    
                        for ext_list in &extensions {
                            let target_dir = match ext_list {
                                &IMAGE_EXTENSIONS => "sorted_images",
                                &AUDIO_EXTENSIONS => "sorted_audios",
                                &VIDEO_EXTENSIONS => "sorted_videos",
                                &DOCUMENT_EXTENSIONS => "sorted_documents",
                                &ARCHIVE_EXTENSIONS => "sorted_archives",
                                _ => unreachable!(),
                            };
    
                            let target_dir_full = source_dir_full.join(target_dir);
                            if let Err(err) = move_files(
                                source_dir_full.to_str().unwrap(),
                                target_dir_full.to_str().unwrap(),
                                *ext_list,
                                self.recursive,
                            ) {
                                self.output_message = format!("Error: {}", err);
                                return;
                            }
                        }
                    }
                    _ => {
                        let target_dir = match self.file_type {
                            FileType::Image => "sorted_images",
                            FileType::Audio => "sorted_audios",
                            FileType::Video => "sorted_videos",
                            FileType::Document => "sorted_documents",
                            FileType::Archive => "sorted_archives",
                            _ => unreachable!(),
                        };
    
                        let target_dir_full = source_dir_full.join(target_dir);
    
                        let extensions = match self.file_type {
                            FileType::Image => IMAGE_EXTENSIONS,
                            FileType::Audio => AUDIO_EXTENSIONS,
                            FileType::Video => VIDEO_EXTENSIONS,
                            FileType::Document => DOCUMENT_EXTENSIONS,
                            FileType::Archive => ARCHIVE_EXTENSIONS,
                            _ => unreachable!(),
                        };
    
                        if let Err(err) = move_files(
                            source_dir_full.to_str().unwrap(),
                            target_dir_full.to_str().unwrap(),
                            extensions,
                            self.recursive,
                        ) {
                            self.output_message = format!("Error: {}", err);
                        } else {
                            self.output_message = "Tiedostot siirretty.".to_string();
                        }
                    }
                }
            }
            Message::FileTypeSelected(file_type) => {
                self.file_type = file_type;
            }
            Message::RecursiveToggleChanged(recursive) => {
                self.recursive = recursive;
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

        let message_text = Text::new(self.output_message.clone());

        let content = Column::new()
            .push(Text::new("Valitse tiedostotyyppi:"))
            .push(file_type_pick_list)
            .push(recursive_checkbox)
            .push(input)
            .push(button)
            .push(message_text);

        content.into()
    }
}

fn main() {
    MyGUI::run(Default::default());
}

