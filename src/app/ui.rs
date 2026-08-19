use iced::{
    Alignment, Element, Length,
    widget::{Container, Space, button, column, progress_bar, row, stack, text, text_input},
};
use iced_wry::webview;

use super::App;
use super::message::Message;

impl App {
    fn view_sidebar(&self) -> Element<'_, Message> {
        let mut tab_buttons = column!().spacing(5);
        for tab in &self.tabs {
            let title = tab.title.as_deref().unwrap_or("New Tab");
            let is_active = self.active_tab_id == Some(tab.id);

            let mut tab_row = row![
                button(text(title).size(14).width(Length::Fill))
                    .style(if is_active {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .on_press(Message::TabSelected(tab.id))
                    .width(Length::Fill),
            ]
            .spacing(5)
            .align_y(Alignment::Center);

            if self.tabs.len() > 1 {
                tab_row = tab_row.push(
                    button(text("X").size(12))
                        .style(button::danger)
                        .on_press(Message::CloseTab(tab.id)),
                );
            }

            tab_buttons = tab_buttons.push(tab_row);
        }

        Container::new(column![
            button(text("+ New Tab").width(Length::Fill).center())
                .on_press(Message::NewTab)
                .width(Length::Fill),
            Space::new().height(Length::Fixed(10.0)),
            tab_buttons
        ])
        .padding(10)
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(30, 30, 35))),
            ..Default::default()
        })
        .into()
    }

    fn view_navbar(&self) -> Element<'_, Message> {
        let nav_container = Container::new(
            row![
                button("Back")
                    .style(button::secondary)
                    .on_press(Message::BackPressed),
                button("Forward")
                    .style(button::secondary)
                    .on_press(Message::ForwardPressed),
                button("Reload")
                    .style(button::secondary)
                    .on_press(Message::ReloadPressed),
                text_input("Enter URL", &self.url_input)
                    .on_input(Message::UrlChanged)
                    .on_submit(Message::GoPressed)
                    .padding(8),
            ]
            .spacing(10)
            .padding(10)
            .align_y(Alignment::Center),
        )
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(45, 45, 50))),
            ..Default::default()
        });

        if let Some(id) = self.active_tab_id {
            if let Some(&progress) = self.tab_progress.get(&id) {
                return column![
                    nav_container,
                    progress_bar(0.0..=100.0, progress)
                        .girth(Length::Fixed(2.0))
                        .style(|_theme| iced::widget::progress_bar::Style {
                            background: iced::Background::Color(iced::Color::TRANSPARENT),
                            bar: iced::Background::Color(iced::Color::from_rgb8(79, 70, 229)),
                            border: iced::Border::default(),
                        })
                ]
                .into();
            }
        }

        column![
            nav_container,
            Space::new().width(Length::Fill).height(Length::Fixed(2.0))
        ]
        .into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut webview_stack = stack!();

        for tab in &self.tabs {
            if let Some(controller) = self.controllers.get(&tab.id) {
                if controller.is_active() {
                    webview_stack = webview_stack
                        .push(webview(controller).width(Length::Fill).height(Length::Fill));
                } else {
                    webview_stack =
                        webview_stack.push(Space::new().width(Length::Fill).height(Length::Fill));
                }
            }
        }

        row![
            self.view_sidebar(),
            column![
                self.view_navbar(),
                webview_stack.width(Length::Fill).height(Length::Fill),
            ]
            .width(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
