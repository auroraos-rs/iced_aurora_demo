use iced::advanced::text::highlighter::PlainText;
use iced::widget::{
    button, center, checkbox, column, container, pick_list, progress_bar, row, rule, scrollable,
    slider, space, text, text_editor, text_input, toggler, TextEditor,
};
use iced::{keyboard, Task};
use iced::{Center, Element, Fill, Subscription, Theme};

pub fn main() -> iced::Result {
    iced::application(Styling::new, Styling::update, Styling::view)
        .subscription(Styling::subscription)
        .scale_factor(|_| 2.0)
        .window(iced::window::Settings {
            fullscreen: true,
            ..Default::default()
        })
        .theme(Styling::theme)
        .run()
}

struct Styling {
    theme: Theme,
    input_value: String,
    editor_content: text_editor::Content,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    EditorChanged(text_editor::Action),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    PreviousTheme,
    NextTheme,
}

impl Styling {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                theme: Theme::Dark,
                input_value: String::new(),
                editor_content: text_editor::Content::default(),
                slider_value: 0.0,
                checkbox_value: false,
                toggler_value: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        // println!("Update");
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::EditorChanged(action) => {
                self.editor_content.perform(action);
            }
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => {
                self.slider_value = value;
            }
            Message::CheckboxToggled(value) => {
                self.checkbox_value = value;
            }
            Message::TogglerToggled(value) => {
                self.toggler_value = value;
            }
            Message::PreviousTheme | Message::NextTheme => {
                if let Some(current) = Theme::ALL
                    .iter()
                    .position(|candidate| &self.theme == candidate)
                {
                    self.theme = if matches!(message, Message::NextTheme) {
                        Theme::ALL[(current + 1) % Theme::ALL.len()].clone()
                    } else if current == 0 {
                        Theme::ALL
                            .last()
                            .expect("Theme::ALL must not be empty")
                            .clone()
                    } else {
                        Theme::ALL[current - 1].clone()
                    };
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Fill),
        ]
        .spacing(10);

        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let styled_button = |label| {
            button(text(label).width(Fill).center())
                .padding(10)
                .on_press(Message::ButtonPressed)
        };

        let primary = styled_button("Primary");
        let success = styled_button("Success").style(button::success);
        let warning = styled_button("Warning").style(button::secondary);
        let danger = styled_button("Danger").style(button::danger);

        let slider = || slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = || progress_bar(0.0..=100.0, self.slider_value);

        let checkbox = checkbox(self.checkbox_value)
            .label("Check me!")
            .on_toggle(Message::CheckboxToggled);

        let toggler = toggler(self.toggler_value)
            .label("Toggle me!")
            .on_toggle(Message::TogglerToggled)
            .spacing(10);

        let card = {
            container(column![text("Card Example").size(24), slider(), progress_bar(),].spacing(20))
                .width(Fill)
                .padding(20)
                .style(container::bordered_box)
        };

        let text_editor: TextEditor<'_, PlainText, Message> = text_editor(&self.editor_content)
            .placeholder("Type something here...")
            .on_action(Message::EditorChanged)
            .padding(10)
            .size(20);

        let scrollable = scrollable(column!["Scroll me!", space().height(800), "You did it!"])
            .width(Fill)
            .height(100);

        let content = column![
            choose_theme,
            rule::horizontal(5),
            text_input,
            text_editor,
            row![primary, success, warning, danger]
                .spacing(10)
                .align_y(Center),
            slider(),
            progress_bar(),
            row![
                scrollable,
                rule::vertical(5),
                column![checkbox, toggler].spacing(20)
            ]
            .spacing(10)
            .height(100)
            .align_y(Center),
            card
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        center(content).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed {
                modified_key: keyboard::Key::Named(modified_key),
                repeat: false,
                ..
            } = event
            else {
                return None;
            };

            match modified_key {
                keyboard::key::Named::ArrowUp | keyboard::key::Named::ArrowLeft => {
                    Some(Message::PreviousTheme)
                }
                keyboard::key::Named::ArrowDown | keyboard::key::Named::ArrowRight => {
                    Some(Message::NextTheme)
                }
                // keyboard::key::Named::Space => Some(Message::ClearTheme),
                _ => None,
            }
        })
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
