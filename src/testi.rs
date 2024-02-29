use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element,
    HorizontalAlignment, Length, Settings, Text, TextInput, text_input,
};

pub fn main() -> iced::Result {
    GUI::run(Settings::default())
}

struct GUI {
    input_value: String,
    input_state: text_input::State,
    submit_button_state: button::State,
    output_message: String, // Viesti käyttäjälle toiminnon tuloksesta
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SubmitPressed,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                input_value: String::new(),
                input_state: text_input::State::new(),
                submit_button_state: button::State::new(),
                output_message: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Tiedostojen Siirto GUI")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::SubmitPressed => {
                // Tässä voitaisiin kutsua `move_files`-funktiota käyttäen `input_value` arvoa
                // Huom: Todellisessa sovelluksessa, tiedostojen käsittelyn tulisi tapahtua erillisessä säikeessä
                // Tässä esimerkissä tulostamme vain syötteen konsoliin
                self.output_message = format!("Käsitellään hakemistoa: {}", self.input_value);
                println!("Käsitellään hakemistoa: {}", self.input_value);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input_state,
            "Syötä hakemiston polku...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10);

        let button = Button::new(&mut self.submit_button_state, Text::new("Siirrä Tiedostot"))
            .on_press(Message::SubmitPressed)
            .padding(10);

        let message_text = Text::new(&self.output_message)
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new()
            .align_items(Align::Center)
            .push(input)
            .push(button)
            .push(message_text)
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
