//! Main iced application

use crate::api::{CheckResult, EnhanceResult, GeminiClient};
use crate::config::{load_config, save_config, AppConfig};
use crate::core::PresetManager;
use crate::ui::i18n::Language;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
    text_editor, text_input, vertical_space,
};
use iced::{Element, Length, Padding, Subscription, Task, Theme};

// Shortcut commands for different desktop environments
const SWAY_CMD: &str = "bindsym $mod+g exec akkurate -s";
const HYPRLAND_CMD: &str = "bind = SUPER, G, exec, akkurate -s";
const KDE_CMD: &str = "akkurate -s";

/// Application state
pub struct App {
    // Core state
    config: AppConfig,
    gemini_client: Option<GeminiClient>,
    preset_manager: PresetManager,

    // UI state
    current_view: View,
    input_content: text_editor::Content,
    result_text: String,
    selected_preset: String,
    is_loading: bool,
    error_message: Option<String>,
    show_setup_guide: bool,

    // Settings state
    api_key_input: String,
    theme_preference: String,
    language: Language,

    // Clipboard message
    clipboard_msg: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum View {
    #[default]
    Main,
    Settings,
    Help,
    Popup,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavigateTo(View),

    // Input handling
    InputChanged(text_editor::Action),
    PresetSelected(String),

    // Actions
    CheckGrammar,
    EnhanceText,
    CopyResult,
    PasteFromClipboard,
    ClearAll,
    CopyCommand(String),

    // Auto actions
    PasteAndCheck,
    PasteAndEnhance,

    // Async results
    CheckComplete(Result<CheckResult, String>),
    EnhanceComplete(Result<EnhanceResult, String>),

    // Settings
    ApiKeyInputChanged(String),
    SaveApiKey,
    ThemeChanged(String),
    LanguageChanged(Language),
    ToggleSetupGuide,

    // Clear clipboard message
    ClearClipboardMsg,
}

/// Flags passed to the application on startup
#[derive(Debug, Clone, Default)]
pub struct AppFlags {
    pub initial_text: Option<String>,
    pub auto_check: bool,
    pub auto_enhance: bool,
    pub check_clipboard: bool,
    pub enhance_clipboard: bool,
}

impl App {
    pub fn new(flags: AppFlags) -> (Self, Task<Message>) {
        let config = load_config().unwrap_or_default();
        let show_setup_guide = config.api.gemini_key.is_empty();

        let gemini_client = if !config.api.gemini_key.is_empty() {
            Some(GeminiClient::new(config.api.gemini_key.clone()))
        } else {
            None
        };

        let mut preset_manager = PresetManager::new();
        if let Some(config_dir) = crate::config::config_path() {
            if let Some(parent) = config_dir.parent() {
                let presets_path = parent.join("presets.toml");
                let _ = preset_manager.load_custom_presets(&presets_path);
            }
        }

        let language = match config.preferences.language.as_str() {
            "english" => Language::English,
            _ => Language::Chinese,
        };

        let initial_view = if flags.check_clipboard || flags.enhance_clipboard {
            View::Popup
        } else {
            View::Main
        };

        let mut app = Self {
            selected_preset: config.preferences.default_preset.clone(),
            theme_preference: config.preferences.theme.clone(),
            api_key_input: config.api.gemini_key.clone(),
            language,
            config,
            gemini_client,
            preset_manager,
            current_view: initial_view,
            input_content: text_editor::Content::new(),
            result_text: String::new(),
            is_loading: false,
            error_message: None,
            show_setup_guide,
            clipboard_msg: None,
        };

        // Handle auto-actions from flags
        let task = if let Some(ref text) = flags.initial_text {
            // Text provided directly via CLI
            app.input_content = text_editor::Content::with_text(text);
            if flags.auto_check {
                Task::perform(async {}, |_| Message::CheckGrammar)
            } else if flags.auto_enhance {
                Task::perform(async {}, |_| Message::EnhanceText)
            } else {
                Task::none()
            }
        } else if flags.check_clipboard {
            Task::perform(async {}, |_| Message::PasteAndCheck)
        } else if flags.enhance_clipboard {
            Task::perform(async {}, |_| Message::PasteAndEnhance)
        } else {
            Task::none()
        };

        (app, task)
    }

    pub fn theme(&self) -> Theme {
        crate::ui::theme::get_theme(&self.theme_preference)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn s(&self) -> &'static crate::ui::i18n::Strings {
        self.language.strings()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(view) => {
                self.current_view = view;
                self.clipboard_msg = None;
                Task::none()
            }

            Message::InputChanged(action) => {
                self.input_content.perform(action);
                Task::none()
            }

            Message::PresetSelected(preset) => {
                self.selected_preset = preset;
                Task::none()
            }

            Message::CheckGrammar => {
                if let Some(client) = &self.gemini_client {
                    let text = self.input_content.text();
                    if text.trim().is_empty() {
                        self.error_message = Some(self.s().enter_text_check.to_string());
                        return Task::none();
                    }

                    self.is_loading = true;
                    self.error_message = None;
                    let client = client.clone();
                    let lang = self.language.display_name();

                    Task::perform(
                        async move { client.check_grammar(&text, lang).await },
                        |result| Message::CheckComplete(result.map_err(|e| e.to_string())),
                    )
                } else {
                    self.error_message = Some(self.s().api_not_configured.to_string());
                    Task::none()
                }
            }

            Message::EnhanceText => {
                if let Some(client) = &self.gemini_client {
                    let text = self.input_content.text();
                    if text.trim().is_empty() {
                        self.error_message = Some(self.s().enter_text_enhance.to_string());
                        return Task::none();
                    }

                    if let Some(preset) = self.preset_manager.get(&self.selected_preset).cloned() {
                        self.is_loading = true;
                        self.error_message = None;
                        let client = client.clone();
                        let lang = self.language.display_name();

                        Task::perform(
                            async move { client.enhance_text(&text, &preset, lang).await },
                            |result| Message::EnhanceComplete(result.map_err(|e| e.to_string())),
                        )
                    } else {
                        self.error_message = Some(self.s().invalid_preset.to_string());
                        Task::none()
                    }
                } else {
                    self.error_message = Some(self.s().api_not_configured.to_string());
                    Task::none()
                }
            }

            Message::CheckComplete(result) => {
                self.is_loading = false;
                match result {
                    Ok(check_result) => {
                        let s = self.s();
                        let mut output = String::new();
                        if check_result.issues.is_empty() {
                            output.push_str(s.no_issues);
                            output.push_str("\n\n");
                        } else {
                            output.push_str(
                                &s.found_issues
                                    .replace("{}", &check_result.issues.len().to_string()),
                            );
                            output.push_str("\n\n");
                            for (i, issue) in check_result.issues.iter().enumerate() {
                                output.push_str(&format!(
                                    "{}. \"{}\" -> \"{}\"\n   {} ({})\n\n",
                                    i + 1,
                                    issue.original,
                                    issue.corrected,
                                    issue.explanation,
                                    issue.rule
                                ));
                            }
                        }
                        output.push_str("---\n");
                        output.push_str(s.corrected_text);
                        output.push_str("\n\n");
                        output.push_str(&check_result.corrected_text);
                        self.result_text = output;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("{}: {}", self.s().error_prefix, e));
                    }
                }
                Task::none()
            }

            Message::EnhanceComplete(result) => {
                self.is_loading = false;
                match result {
                    Ok(enhance_result) => {
                        let s = self.s();
                        let mut output = String::new();
                        output.push_str(s.enhanced_text);
                        output.push_str("\n\n");
                        output.push_str(&enhance_result.enhanced_text);
                        output.push_str("\n\n---\n");
                        output.push_str(s.changes_made);
                        output.push_str("\n\n");
                        for change in &enhance_result.changes_made {
                            output.push_str(&format!("* {}\n", change));
                        }
                        self.result_text = output;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("{}: {}", self.s().error_prefix, e));
                    }
                }
                Task::none()
            }

            Message::CopyResult => {
                if !self.result_text.is_empty() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        let _ = clipboard.set_text(&self.result_text);
                    }
                }
                Task::none()
            }

            Message::CopyCommand(cmd) => {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(&cmd);
                    self.clipboard_msg = Some(if self.language == Language::Chinese {
                        "已复制!".to_string()
                    } else {
                        "Copied!".to_string()
                    });
                }
                Task::none()
            }

            Message::PasteFromClipboard => {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    if let Ok(text) = clipboard.get_text() {
                        self.input_content = text_editor::Content::with_text(&text);
                    }
                }
                Task::none()
            }

            Message::ClearAll => {
                self.input_content = text_editor::Content::new();
                self.result_text.clear();
                self.error_message = None;
                Task::none()
            }

            Message::ApiKeyInputChanged(key) => {
                self.api_key_input = key;
                Task::none()
            }

            Message::SaveApiKey => {
                self.config.api.gemini_key = self.api_key_input.clone();
                if let Err(e) = save_config(&self.config) {
                    self.error_message = Some(format!("{}: {}", self.s().save_failed, e));
                } else {
                    self.gemini_client = Some(GeminiClient::new(self.api_key_input.clone()));
                    self.show_setup_guide = false;
                    self.error_message = None;
                }
                Task::none()
            }

            Message::ThemeChanged(theme) => {
                self.theme_preference = theme.clone();
                self.config.preferences.theme = theme;
                let _ = save_config(&self.config);
                Task::none()
            }

            Message::LanguageChanged(lang) => {
                self.language = lang;
                self.config.preferences.language = match lang {
                    Language::Chinese => "chinese".to_string(),
                    Language::English => "english".to_string(),
                };
                let _ = save_config(&self.config);
                Task::none()
            }

            Message::ToggleSetupGuide => {
                self.show_setup_guide = !self.show_setup_guide;
                Task::none()
            }

            Message::ClearClipboardMsg => {
                self.clipboard_msg = None;
                Task::none()
            }

            Message::PasteAndCheck => {
                tracing::info!("PasteAndCheck triggered");

                // Try arboard first, then fall back to wl-paste for Wayland
                let clipboard_text = arboard::Clipboard::new()
                    .and_then(|mut cb| cb.get_text())
                    .or_else(|e| {
                        tracing::info!("arboard failed: {:?}, trying fallback", e);
                        
                        #[cfg(target_os = "linux")]
                        {
                            std::process::Command::new("wl-paste")
                                .arg("--no-newline")
                                .output()
                                .map_err(|e| arboard::Error::Unknown {
                                    description: e.to_string(),
                                })
                                .and_then(|output| {
                                    if output.status.success() {
                                        String::from_utf8(output.stdout).map_err(|e| {
                                            arboard::Error::Unknown {
                                                description: e.to_string(),
                                            }
                                        })
                                    } else {
                                        Err(arboard::Error::ContentNotAvailable)
                                    }
                                })
                        }
                        
                        #[cfg(not(target_os = "linux"))]
                        {
                            Err(e)
                        }
                    });

                match clipboard_text {
                    Ok(text) if !text.is_empty() => {
                        tracing::info!("Clipboard text length: {} chars", text.len());
                        self.input_content = text_editor::Content::with_text(&text);
                        return Task::perform(async {}, |_| Message::CheckGrammar);
                    }
                    Ok(_) => {
                        self.error_message = Some("剪切板为空".to_string());
                    }
                    Err(e) => {
                        tracing::warn!("All clipboard methods failed: {:?}", e);
                        self.error_message = Some(format!("剪切板读取失败: {}", e));
                    }
                }
                Task::none()
            }

            Message::PasteAndEnhance => {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    if let Ok(text) = clipboard.get_text() {
                        self.input_content = text_editor::Content::with_text(&text);
                        return Task::perform(async {}, |_| Message::EnhanceText);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.current_view {
            View::Main => self.view_main(),
            View::Settings => self.view_settings(),
            View::Help => self.view_help(),
            View::Popup => self.view_popup(),
        };

        if self.current_view == View::Popup {
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
                .into()
        } else {
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(Padding::new(24.0))
                .into()
        }
    }

    fn nav_button<'a>(&self, label: &'a str, target: View) -> iced::widget::Button<'a, Message> {
        let is_active = self.current_view == target;
        let btn = button(text(label).size(14));
        if is_active {
            btn.style(button::primary)
        } else {
            btn.style(button::secondary)
        }
        .on_press(Message::NavigateTo(target))
    }

    fn view_main(&self) -> Element<'_, Message> {
        let s = self.s();

        // Modern navigation bar
        let nav = row![
            self.nav_button(s.nav_main, View::Main),
            self.nav_button(s.nav_settings, View::Settings),
            self.nav_button(s.nav_help, View::Help),
            horizontal_space(),
            container(
                text(format!(
                    "{}: {}",
                    s.current_style,
                    s.preset_display_name(&self.selected_preset)
                ))
                .size(13)
            )
            .padding(Padding::from([6, 12]))
            .style(container::rounded_box),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        // Setup guide banner
        let setup_banner: Element<Message> = if self.show_setup_guide {
            container(
                column![
                    row![
                        text(s.welcome_title).size(16),
                        horizontal_space(),
                        button(text("x").size(12))
                            .style(button::text)
                            .on_press(Message::ToggleSetupGuide),
                    ]
                    .align_y(iced::Alignment::Center),
                    text(s.welcome_msg).size(13),
                    text(s.welcome_instruction).size(13),
                    vertical_space().height(8),
                    button(text(s.go_to_settings).size(13))
                        .style(button::primary)
                        .on_press(Message::NavigateTo(View::Settings)),
                ]
                .spacing(6),
            )
            .padding(16)
            .style(container::bordered_box)
            .width(Length::Fill)
            .into()
        } else {
            column![].into()
        };

        // Preset selector
        let preset_keys: Vec<String> = self.preset_manager.keys().into_iter().cloned().collect();

        let preset_picker = row![
            text(format!("{}:", s.style_preset)).size(13),
            pick_list(
                preset_keys,
                Some(self.selected_preset.clone()),
                Message::PresetSelected
            )
            .placeholder(s.select_preset)
            .text_size(13),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        // Input area with modern styling
        let input_area = column![
            text(s.input_text).size(14),
            container(
                text_editor(&self.input_content)
                    .on_action(Message::InputChanged)
                    .height(Length::Fill)
            )
            .height(Length::FillPortion(1))
            .style(container::bordered_box),
            row![
                button(text(s.paste).size(13))
                    .style(button::secondary)
                    .on_press(Message::PasteFromClipboard),
                button(text(s.clear).size(13))
                    .style(button::secondary)
                    .on_press(Message::ClearAll),
                horizontal_space(),
                preset_picker,
            ]
            .spacing(8),
        ]
        .spacing(8);

        // Action buttons with modern styling
        let action_buttons = row![
            button(
                text(if self.is_loading {
                    s.processing
                } else {
                    s.check_grammar
                })
                .size(14)
            )
            .style(button::primary)
            .padding(Padding::from([10, 20]))
            .on_press_maybe(if self.is_loading {
                None
            } else {
                Some(Message::CheckGrammar)
            }),
            button(
                text(if self.is_loading {
                    s.processing
                } else {
                    s.enhance_text
                })
                .size(14)
            )
            .style(button::success)
            .padding(Padding::from([10, 20]))
            .on_press_maybe(if self.is_loading {
                None
            } else {
                Some(Message::EnhanceText)
            }),
        ]
        .spacing(12);

        // Error message
        let error_view: Element<Message> = if let Some(ref err) = self.error_message {
            container(text(format!("! {}", err)).size(13))
                .padding(12)
                .style(container::bordered_box)
                .width(Length::Fill)
                .into()
        } else {
            column![].into()
        };

        // Result area with modern styling
        let result_area = column![
            row![
                text(s.result).size(14),
                horizontal_space(),
                button(text(s.copy_result).size(13))
                    .style(button::secondary)
                    .on_press(Message::CopyResult),
            ]
            .align_y(iced::Alignment::Center),
            container(scrollable(
                container(text(&self.result_text).size(13))
                    .padding(12)
                    .width(Length::Fill)
            ))
            .height(180)
            .style(container::bordered_box),
        ]
        .spacing(8);

        column![
            nav,
            vertical_space().height(16),
            setup_banner,
            input_area,
            vertical_space().height(12),
            action_buttons,
            error_view,
            vertical_space().height(12),
            result_area,
        ]
        .spacing(8)
        .into()
    }

    fn view_settings(&self) -> Element<'_, Message> {
        let s = self.s();

        let nav = row![
            self.nav_button(s.nav_main, View::Main),
            self.nav_button(s.nav_settings, View::Settings),
            self.nav_button(s.nav_help, View::Help),
        ]
        .spacing(8);

        let api_section = column![
            text(s.api_config).size(18),
            vertical_space().height(8),
            text(s.enter_api_key).size(13),
            row![
                text_input(s.api_key_placeholder, &self.api_key_input)
                    .on_input(Message::ApiKeyInputChanged)
                    .secure(true)
                    .size(13)
                    .width(Length::FillPortion(3)),
                button(text(s.save).size(13))
                    .style(button::primary)
                    .on_press(Message::SaveApiKey),
            ]
            .spacing(8),
            text(s.get_api_key).size(12),
        ]
        .spacing(8);

        let theme_options = vec!["dark".to_string(), "light".to_string()];
        let lang_options = Language::all();

        let appearance_section = column![
            text(s.appearance).size(18),
            vertical_space().height(8),
            row![
                text(format!("{}:", s.theme)).size(13),
                pick_list(
                    theme_options,
                    Some(self.theme_preference.clone()),
                    Message::ThemeChanged
                )
                .text_size(13),
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
            row![
                text(format!("{}:", s.language)).size(13),
                pick_list(lang_options, Some(self.language), Message::LanguageChanged)
                    .text_size(13),
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(8);

        let error_view: Element<Message> = if let Some(ref err) = self.error_message {
            container(text(format!("! {}", err)).size(13))
                .padding(12)
                .style(container::bordered_box)
                .into()
        } else {
            column![].into()
        };

        column![
            nav,
            vertical_space().height(24),
            api_section,
            vertical_space().height(24),
            horizontal_rule(1),
            vertical_space().height(24),
            appearance_section,
            error_view,
        ]
        .spacing(8)
        .into()
    }

    fn view_help(&self) -> Element<'_, Message> {
        let s = self.s();

        let nav = row![
            self.nav_button(s.nav_main, View::Main),
            self.nav_button(s.nav_settings, View::Settings),
            self.nav_button(s.nav_help, View::Help),
            horizontal_space(),
            if let Some(ref msg) = self.clipboard_msg {
                text(msg).size(13)
            } else {
                text("").size(13)
            },
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        // Copyable command row helper
        let cmd_row = |label: &'static str, cmd: &'static str| -> Element<'_, Message> {
            column![
                text(label).size(14),
                row![
                    container(text(cmd).size(12))
                        .padding(Padding::from([8, 12]))
                        .style(container::bordered_box)
                        .width(Length::Fill),
                    button(text(s.copy_cmd).size(12))
                        .style(button::secondary)
                        .on_press(Message::CopyCommand(cmd.to_string())),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(4)
            .into()
        };

        let shortcuts_section = column![
            text(s.shortcut_title).size(18),
            vertical_space().height(4),
            text(s.shortcut_wayland_note).size(13),
            text(s.shortcut_config_note).size(13),
            vertical_space().height(12),
            cmd_row(s.shortcut_sway, SWAY_CMD),
            vertical_space().height(8),
            cmd_row(s.shortcut_hyprland, HYPRLAND_CMD),
            vertical_space().height(8),
            cmd_row(s.shortcut_kde, KDE_CMD),
            vertical_space().height(8),
            text(s.shortcut_gnome).size(13),
        ]
        .spacing(4);

        let usage_section = column![
            text(s.usage_guide).size(18),
            vertical_space().height(8),
            text(s.usage_step1).size(13),
            text(s.usage_step2).size(13),
            text(s.usage_step3).size(13),
            text(s.usage_step4).size(13),
            text(s.usage_step5).size(13),
        ]
        .spacing(4);

        let preset_section = column![
            text(s.preset_guide).size(18),
            vertical_space().height(8),
            text(s.preset_casual).size(13),
            text(s.preset_business).size(13),
            text(s.preset_academic).size(13),
            text(s.preset_creative).size(13),
        ]
        .spacing(4);

        scrollable(
            column![
                nav,
                vertical_space().height(24),
                usage_section,
                vertical_space().height(24),
                horizontal_rule(1),
                vertical_space().height(24),
                preset_section,
                vertical_space().height(24),
                horizontal_rule(1),
                vertical_space().height(24),
                shortcuts_section,
                vertical_space().height(40),
            ]
            .spacing(4),
        )
        .into()
    }

    fn view_popup(&self) -> Element<'_, Message> {
        let s = self.s();

        // Compact result area
        let result_area: Element<Message> = if !self.result_text.is_empty() {
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
                container(scrollable(
                    container(text(&self.result_text).size(13))
                        .padding(8)
                        .width(Length::Fill)
                ))
                .height(Length::Fill) // Fill remaining space
                .style(container::bordered_box),
            ]
            .spacing(4)
            .into()
        } else {
            column![].into()
        };

        // Compact input area
        let input_area = column![
            container(
                text_editor(&self.input_content)
                    .on_action(Message::InputChanged)
                    .height(Length::Fixed(100.0))
            )
            .style(container::bordered_box),
            row![
                // Compact preset picker
                text(format!("{}:", s.style_preset)).size(12),
                pick_list(
                    self.preset_manager
                        .keys()
                        .into_iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                    Some(self.selected_preset.clone()),
                    Message::PresetSelected
                )
                .text_size(12)
                .padding(4),
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
            button(
                text(if self.is_loading {
                    s.processing
                } else {
                    s.check_grammar
                })
                .size(13)
            )
            .style(button::primary)
            .width(Length::Fill)
            .padding(Padding::from([8, 0]))
            .on_press_maybe(if self.is_loading {
                None
            } else {
                Some(Message::CheckGrammar)
            }),
            button(
                text(if self.is_loading {
                    s.processing
                } else {
                    s.enhance_text
                })
                .size(13)
            )
            .style(button::success)
            .width(Length::Fill)
            .padding(Padding::from([8, 0]))
            .on_press_maybe(if self.is_loading {
                None
            } else {
                Some(Message::EnhanceText)
            }),
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

        column![input_area, action_buttons, error_view, result_area,]
            .spacing(8)
            .into()
    }
}
