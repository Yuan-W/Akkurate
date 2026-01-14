
    fn view_popup(&self) -> Element<'_, Message> {
        let s = self.s();

        // Compact result area
        let result_area = if !self.result_text.is_empty() {
             column![
                row![
                    text(s.result).size(13),
                    horizontal_space(),
                    button(text(s.copy_result).size(12))
                        .style(button::secondary)
                        .padding(Padding::from([4, 8]))
                        .on_press(Message::CopyResult),
                ]
                .align_y(iced::Alignment::Center),
                container(
                    scrollable(
                        container(text(&self.result_text).size(13))
                            .padding(8)
                            .width(Length::Fill)
                    )
                )
                .height(Length::FillPortion(2)) // More space for result
                .style(container::bordered_box),
            ]
            .spacing(4)
        } else {
            column![].into()
        };

        // Compact input area
        let input_area = column![
             container(
                text_editor(&self.input_content)
                    .on_action(Message::InputChanged)
            )
            .height(Length::FillPortion(1))
            .style(container::bordered_box),
            row![
                 // Compact preset picker
                 text(format!("{}:", s.style_preset)).size(12),
                 pick_list(
                    self.preset_manager.keys().into_iter().cloned().collect::<Vec<_>>(),
                    Some(self.selected_preset.clone()),
                    Message::PresetSelected
                 ).text_size(12).padding(4),
                 horizontal_space(),
                 button(text(s.clear).size(12))
                    .style(button::secondary)
                    .padding(Padding::from([4, 8]))
                    .on_press(Message::ClearAll),
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(4);

        // Compact actions
        let action_buttons = row![
            button(text(if self.is_loading { s.processing } else { s.check_grammar }).size(13))
                .style(button::primary)
                .width(Length::Fill)
                .padding(Padding::from([8, 0]))
                .on_press_maybe(if self.is_loading { None } else { Some(Message::CheckGrammar) }),
            button(text(if self.is_loading { s.processing } else { s.enhance_text }).size(13))
                .style(button::success)
                .width(Length::Fill)
                .padding(Padding::from([8, 0]))
                .on_press_maybe(if self.is_loading { None } else { Some(Message::EnhanceText) }),
        ]
        .spacing(8);

        // Error message
        let error_view: Element<Message> = if let Some(ref err) = self.error_message {
            container(text(format!("! {}", err)).size(12))
                .padding(8)
                .style(container::bordered_box)
                .width(Length::Fill)
                .into()
        } else {
            column![].into()
        };

        column![
            input_area,
            action_buttons,
            error_view,
            result_area,
        ]
        .spacing(8)
        .into()
    }
