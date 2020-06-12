use iced::{
    button, scrollable, text_input, Align, Button, Column, Container, Element, Length, Row, Sandbox, Scrollable, Settings, Text,
    TextInput,
};
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
    output_text: String,
    scroll: scrollable::State,
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
                let output = execute_command(&self.command);
                handle_output(self, output);
            }
            Message::CommandChanged(command) => {
                self.command = command;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let fields = Row::new().padding(20).push(
            TextInput::new(
                &mut self.command_input,
                "Type command here...",
                &self.command,
                Message::CommandChanged,
            )
            .padding(12),
        );

        let controls = Row::new().padding(20).push(
            Button::new(&mut self.execute_button, Text::new("Execute"))
                .on_press(Message::ButtonPressed)
                .style(style::Button::Primary),
        );

        let output = Row::new().padding(20).push(
            Text::new(&self.output_text).size(16)
        );

        let content = Column::new()
            .align_items(Align::Center)
            .push(fields)
            .push(controls)
            .push(output);

        Container::new(
            Scrollable::new(&mut self.scroll)
                .width(Length::Fill)
                .height(Length::Fill)
                .push(content)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Container::Dark)
        .into()
    }
}

fn execute_command(command: &str) -> std::process::Output {
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

    output
}

fn handle_output(app: &mut App, output: std::process::Output) {
    app.output_text = format!("Status: {}\n\n", output.status);
    if output.stdout.len() > 0 {
        let out_string = format!("Output:\n{}\n\n", String::from_utf8(output.stdout).unwrap());
        app.output_text.push_str(out_string.as_str());
    }
    if output.stderr.len() > 0 {
        let err_string = format!("Errors:\n{}\n\n", String::from_utf8(output.stderr).unwrap());
        app.output_text.push_str(err_string.as_str());
    }
    if !output.status.success() {
        app.output_text.push_str("Command was not successful");
        return;
    }
}

mod style {
    use iced::{button, container, Background, Color, Vector};

    pub enum Container {
        Light,
        Dark,
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(match self {
                    Container::Light => Color::WHITE,
                    Container::Dark => Color::from_rgb(0.1, 0.1, 0.1),
                })),
                text_color: Some(match self {
                    Container::Light => Color::BLACK,
                    Container::Dark => Color::WHITE,
                }),
                ..container::Style::default()
            }
        }
    }

    pub enum Button {
        Primary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_radius: 4,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb(0.9, 0.9, 0.9),
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
