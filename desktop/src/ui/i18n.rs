//! Internationalization (i18n) support

use serde::{Deserialize, Serialize};

/// Supported languages
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    Chinese,
    English,
}

impl Language {
    pub fn strings(&self) -> &'static Strings {
        match self {
            Language::Chinese => &CHINESE,
            Language::English => &ENGLISH,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
        }
    }

    pub fn all() -> Vec<Language> {
        vec![Language::Chinese, Language::English]
    }
}

impl Strings {
    /// Get localized display name for a preset key
    pub fn preset_display_name(&self, key: &str) -> String {
        match key {
            "casual" => self.preset_name_casual.to_string(),
            "business" => self.preset_name_business.to_string(),
            "academic" => self.preset_name_academic.to_string(),
            "creative" => self.preset_name_creative.to_string(),
            _ => key.to_string(),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// All UI strings for the application
pub struct Strings {
    // Navigation
    pub nav_main: &'static str,
    pub nav_settings: &'static str,
    pub nav_help: &'static str,

    // Main view
    pub current_style: &'static str,
    pub welcome_title: &'static str,
    pub welcome_msg: &'static str,
    pub welcome_instruction: &'static str,
    pub go_to_settings: &'static str,
    pub style_preset: &'static str,
    pub select_preset: &'static str,
    pub input_text: &'static str,
    pub paste: &'static str,
    pub clear: &'static str,
    pub processing: &'static str,
    pub check_grammar: &'static str,
    pub enhance_text: &'static str,
    pub result: &'static str,
    pub copy_result: &'static str,

    // Results
    pub no_issues: &'static str,
    pub found_issues: &'static str,
    pub corrected_text: &'static str,
    pub enhanced_text: &'static str,
    pub changes_made: &'static str,

    // Errors
    pub enter_text_check: &'static str,
    pub enter_text_enhance: &'static str,
    pub api_not_configured: &'static str,
    pub invalid_preset: &'static str,
    pub error_prefix: &'static str,
    pub save_failed: &'static str,

    // Settings
    pub api_config: &'static str,
    pub enter_api_key: &'static str,
    pub api_key_placeholder: &'static str,
    pub save: &'static str,
    pub get_api_key: &'static str,
    pub appearance: &'static str,
    pub theme: &'static str,
    pub language: &'static str,

    // Help
    pub shortcut_title: &'static str,
    pub shortcut_wayland_note: &'static str,
    pub shortcut_config_note: &'static str,
    pub shortcut_sway: &'static str,
    pub shortcut_hyprland: &'static str,
    pub shortcut_gnome: &'static str,
    pub shortcut_kde: &'static str,
    pub add_to_config: &'static str,
    pub copy_cmd: &'static str,
    pub usage_guide: &'static str,
    pub usage_step1: &'static str,
    pub usage_step2: &'static str,
    pub usage_step3: &'static str,
    pub usage_step4: &'static str,
    pub usage_step5: &'static str,
    pub preset_guide: &'static str,
    pub preset_casual: &'static str,
    pub preset_business: &'static str,
    pub preset_academic: &'static str,
    pub preset_creative: &'static str,

    // Preset display names for dropdown
    pub preset_name_casual: &'static str,
    pub preset_name_business: &'static str,
    pub preset_name_academic: &'static str,
    pub preset_name_creative: &'static str,
}

const CHINESE: Strings = Strings {
    // Navigation
    nav_main: "主页",
    nav_settings: "设置",
    nav_help: "帮助",

    // Main view
    current_style: "当前风格",
    welcome_title: "欢迎使用 Akkurate!",
    welcome_msg: "开始使用前，请先配置您的 Gemini API 密钥。",
    welcome_instruction: "前往「设置」> 输入 API 密钥 > 保存",
    go_to_settings: "前往设置",
    style_preset: "文风预设",
    select_preset: "选择预设",
    input_text: "输入文本:",
    paste: "[粘贴]",
    clear: "[清空]",
    processing: "处理中...",
    check_grammar: "[检查语法]",
    enhance_text: "[润色文本]",
    result: "结果:",
    copy_result: "[复制结果]",

    // Results
    no_issues: "[OK] 没有发现语法问题!",
    found_issues: "发现 {} 个问题:",
    corrected_text: "修正后的文本:",
    enhanced_text: "润色后的文本:",
    changes_made: "修改说明:",

    // Errors
    enter_text_check: "请输入要检查的文本",
    enter_text_enhance: "请输入要润色的文本",
    api_not_configured: "API 密钥未配置，请前往设置",
    invalid_preset: "无效的风格预设",
    error_prefix: "错误",
    save_failed: "保存配置失败",

    // Settings
    api_config: "API 配置",
    enter_api_key: "请输入您的 Gemini API 密钥:",
    api_key_placeholder: "API 密钥",
    save: "[保存]",
    get_api_key: "获取密钥: https://aistudio.google.com/apikey",
    appearance: "外观设置",
    theme: "主题",
    language: "语言",

    // Help
    shortcut_title: "快捷键设置",
    shortcut_wayland_note: "由于 Wayland 安全限制，应用无法直接注册全局快捷键。",
    shortcut_config_note: "您需要在桌面环境中手动配置快捷键:",
    shortcut_sway: "Sway (~/.config/sway/config):",
    shortcut_hyprland: "Hyprland (~/.config/hypr/hyprland.conf):",
    shortcut_gnome: "GNOME: 设置 > 键盘 > 自定义快捷键",
    shortcut_kde: "KDE Plasma: 系统设置 > 快捷键 > 自定义快捷键",
    add_to_config: "添加到配置文件",
    copy_cmd: "[复制]",
    usage_guide: "使用指南",
    usage_step1: "1. 在任意应用中选中英文文本（高亮即可）",
    usage_step2: "2. 按热键（如 Super+G）触发 akkurate -s",
    usage_step3: "3. 自动检查语法并显示结果",
    usage_step4: "4. 或使用主界面输入/粘贴文本进行检查",
    usage_step5: "5. 复制结果到其他地方使用",
    preset_guide: "文风预设说明",
    preset_casual: "casual（日常）: 友好随意，适合聊天、社交媒体",
    preset_business: "business（商务）: 专业礼貌，适合邮件、报告",
    preset_academic: "academic（学术）: 正式严谨，适合论文、文档",
    preset_creative: "creative（创意）: 生动表达，适合故事、博客",
    preset_name_casual: "日常",
    preset_name_business: "商务",
    preset_name_academic: "学术",
    preset_name_creative: "创意",
};

const ENGLISH: Strings = Strings {
    // Navigation
    nav_main: "Main",
    nav_settings: "Settings",
    nav_help: "Help",

    // Main view
    current_style: "Current Style",
    welcome_title: "Welcome to Akkurate!",
    welcome_msg: "To get started, configure your Gemini API key.",
    welcome_instruction: "Go to Settings > Enter API key > Save",
    go_to_settings: "Go to Settings",
    style_preset: "Style Preset",
    select_preset: "Select preset",
    input_text: "Input Text:",
    paste: "[Paste]",
    clear: "[Clear]",
    processing: "Processing...",
    check_grammar: "[Check Grammar]",
    enhance_text: "[Enhance Text]",
    result: "Result:",
    copy_result: "[Copy Result]",

    // Results
    no_issues: "[OK] No grammar issues found!",
    found_issues: "Found {} issue(s):",
    corrected_text: "Corrected text:",
    enhanced_text: "Enhanced text:",
    changes_made: "Changes made:",

    // Errors
    enter_text_check: "Please enter some text to check",
    enter_text_enhance: "Please enter some text to enhance",
    api_not_configured: "API key not configured. Go to Settings.",
    invalid_preset: "Invalid preset selected",
    error_prefix: "Error",
    save_failed: "Failed to save config",

    // Settings
    api_config: "API Configuration",
    enter_api_key: "Enter your Gemini API key:",
    api_key_placeholder: "API Key",
    save: "[Save]",
    get_api_key: "Get your key: https://aistudio.google.com/apikey",
    appearance: "Appearance",
    theme: "Theme",
    language: "Language",

    // Help
    shortcut_title: "Keyboard Shortcuts Setup",
    shortcut_wayland_note: "Wayland does not allow apps to register global hotkeys for security.",
    shortcut_config_note: "You need to configure shortcuts in your desktop environment:",
    shortcut_sway: "Sway (~/.config/sway/config):",
    shortcut_hyprland: "Hyprland (~/.config/hypr/hyprland.conf):",
    shortcut_gnome: "GNOME: Settings > Keyboard > Custom Shortcuts",
    shortcut_kde: "KDE Plasma: System Settings > Shortcuts > Custom Shortcuts",
    add_to_config: "Add to config file",
    copy_cmd: "[Copy]",
    usage_guide: "Usage Guide",
    usage_step1: "1. Select English text in any application (just highlight)",
    usage_step2: "2. Press hotkey (e.g., Super+G) to trigger akkurate -s",
    usage_step3: "3. Grammar is auto-checked and results are shown",
    usage_step4: "4. Or use the main interface to input/paste text",
    usage_step5: "5. Copy the results to use elsewhere",
    preset_guide: "Style Presets",
    preset_casual: "casual: Friendly, conversational - for chat, social media",
    preset_business: "business: Professional, polite - for emails, reports",
    preset_academic: "academic: Formal, rigorous - for papers, documentation",
    preset_creative: "creative: Expressive, vivid - for stories, blogs",
    preset_name_casual: "Casual",
    preset_name_business: "Business",
    preset_name_academic: "Academic",
    preset_name_creative: "Creative",
};
