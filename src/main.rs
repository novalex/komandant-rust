use std::process::Command;
use std::io::{self, Write};
use iced::{
    Align,
    Button,
    button,
    Column,
    Element,
    Row,
    Sandbox,
    Settings,
    Text,
};

pub fn main() {
    App::run(Settings::default())
}

#[derive(Default)]
pub struct App {
    // fields: Fields,
    execute_button: button::State,
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
                // self.fields.parse();
                println!( "All systems go!" );
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut controls = Row::new()
            .push(
                Button::new(&mut self.execute_button, Text::new("Execute"))
                    .on_press(Message::ButtonPressed)
                    .style(style::Button::Primary),
            );

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push( controls )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
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

// fn main() {
//     let output = if cfg!(target_os = "windows") {
//         Command::new("cmd")
//                 .args(&["/C", "echo hello"])
//                 .output()
//                 .expect("failed to execute process")
//     } else {
//         Command::new("sh")
//                 .arg("-c")
//                 .arg("echo hello")
//                 .output()
//                 .expect("failed to execute process")
//     };

//     println!("status: {}", output.status);
//     io::stdout().write_all(&output.stdout).unwrap();
//     io::stderr().write_all(&output.stderr).unwrap();

//     assert!(output.status.success());
// }
