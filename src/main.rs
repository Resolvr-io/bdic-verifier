#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use bitcoin::bip32::Xpub;
use iced::widget::{button, column, container, text, text_input};
use iced::{window, Color, Element, Font, Length, Size, Task, Theme};
use sha2::{Digest, Sha256};
use std::str::FromStr;

const DEFAULT_WINDOW_SIZE: Size = Size::new(620.0, 360.0);

/// Main application state.
#[derive(Default)]
struct XpubHasher {
    /// The xpub input from the user.
    xpub_input: String,

    /// Whether the entered xpub is valid.
    is_valid_xpub: bool,

    /// The computed SHA256 hash of the xpub, if any.
    hash_result: Option<String>,
}

/// Message type for handling application events.
#[derive(Debug, Clone)]
enum Message {
    XpubInputChanged(String),
    CopyHash,
}

fn main() -> iced::Result {
    iced::application("Bitcoin Xpub Hasher", update, view)
        .theme(|_| Theme::Light)
        .window(window::Settings {
            size: DEFAULT_WINDOW_SIZE,
            min_size: Some(DEFAULT_WINDOW_SIZE),
            ..Default::default()
        })
        .run()
}

fn update(state: &mut XpubHasher, message: Message) -> Task<Message> {
    match message {
        Message::XpubInputChanged(value) => {
            state.xpub_input = value;

            state.is_valid_xpub = validate_xpub(&state.xpub_input);

            state.hash_result = if state.is_valid_xpub {
                let mut hasher = Sha256::new();
                hasher.update(state.xpub_input.as_bytes());
                let result = hasher.finalize();
                Some(hex::encode(result))
            } else {
                None
            };

            Task::none()
        }
        Message::CopyHash =>
        {
            #[allow(clippy::option_if_let_else)]
            if let Some(ref hash) = state.hash_result {
                iced::clipboard::write(hash.clone())
            } else {
                Task::none()
            }
        }
    }
}

fn view(state: &XpubHasher) -> Element<'_, Message> {
    let input = text_input("Enter Bitcoin xpub...", &state.xpub_input)
        .on_input(Message::XpubInputChanged)
        .padding(10)
        .size(16);

    let title_text = text("Bitcoin Xpub Hasher")
        .size(28)
        .color(Color::from_rgb(0.0, 0.4, 0.8));

    let subtitle_text = text("Enter a valid Bitcoin xpub to automatically compute its SHA256 hash")
        .size(14)
        .color(Color::from_rgb(0.5, 0.5, 0.5));

    let mut content = column![title_text, subtitle_text, input].spacing(15);

    // Add validation status.
    if !state.xpub_input.is_empty() {
        let status_text = if state.is_valid_xpub {
            text("✓ Valid xpub")
                .size(14)
                .color(Color::from_rgb(0.0, 0.7, 0.0))
        } else {
            text("✗ Invalid xpub")
                .size(14)
                .color(Color::from_rgb(0.8, 0.0, 0.0))
        };
        content = content.push(status_text);
    }

    // Display hash result if available.
    if let Some(ref hash) = state.hash_result {
        let hash_display = column![
            text("SHA256 Hash:").size(16),
            container(text(hash).size(14).font(Font::MONOSPACE))
                .padding(10)
                .style(container::rounded_box),
            button("Copy Hash to Clipboard")
                .on_press(Message::CopyHash)
                .padding([10, 20]),
        ]
        .spacing(10);

        content = content.push(hash_display);
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

/// Validates whether the provided string is a valid Bitcoin xpub.
fn validate_xpub(xpub: &str) -> bool {
    Xpub::from_str(xpub).is_ok()
}
