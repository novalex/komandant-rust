use iced::{
    button, text_input, Align, Button, Column, Element, Row, Sandbox, Settings, Text, TextInput,
};
use std::io::{self, Write};
use std::process::Command;

fn main() {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    // fields: Fields,
    command: String,
    command_input: text_input::State,
    execute_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    CommandChanged(String),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Komandant")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::ButtonPressed => {
                execute_command(&self.command);
            }
            Message::CommandChanged(command) => {
                self.command = command;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let fields = Row::new()
            .padding(20)
            .push(
                TextInput::new(&mut self.command_input, "Type command here...", &self.command, Message::CommandChanged)
            );

        let controls = Row::new()
            .padding(20)
            .push(
                Button::new(&mut self.execute_button, Text::new("Execute"))
                    .on_press(Message::ButtonPressed)
                    .style(style::Button::Primary),
            );

        Column::new()
            .align_items(Align::Center)
            .push(fields)
            .push(controls)
            .into()
    }
}

fn execute_command(command: &str) {
    let mut shell = "sh";
    let mut arg = "-c";
    if cfg!(target_os = "windows") {
        shell = "cmd";
        arg = "/C";
    }

    let output = Command::new(shell)
        .args(&[arg, command])
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 4,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
