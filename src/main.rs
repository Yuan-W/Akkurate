use akkurate::ui::{App, AppFlags};
use clap::Parser;
use iced::{self, Font};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const NOTO_SANS_CJK: &[u8] = include_bytes!("../assets/NotoSansCJK-Regular.ttc");

#[derive(Parser, Debug)]
#[command(author, version, about = "Akkurate - Grammar Assistant for Linux")]
struct Args {
    /// Check grammar for selected text (reads PRIMARY selection via wl-paste)
    #[arg(short = 's', long)]
    check_selection: bool,

    /// Check grammar for provided text
    #[arg(long, value_name = "TEXT")]
    check: Option<String>,

    /// Enhance provided text
    #[arg(long, value_name = "TEXT")]
    enhance: Option<String>,
}

/// Read PRIMARY selection using wl-paste
fn get_selection() -> Option<String> {
    std::process::Command::new("wl-paste")
        .args(["--primary", "--no-newline"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .filter(|s| !s.trim().is_empty())
}

fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Akkurate...");

    let args = Args::parse();

    // Compute flags before consuming args
    let has_check = args.check.is_some();
    let has_enhance = args.enhance.is_some();

    // Get text from selection if --check-selection is used
    let initial_text = if args.check_selection {
        match get_selection() {
            Some(text) => {
                tracing::info!("Got selection: {} chars", text.len());
                Some(text)
            }
            None => {
                eprintln!("Error: No text selected. Please select text before running.");
                std::process::exit(1);
            }
        }
    } else {
        args.check.or(args.enhance)
    };

    let is_popup = initial_text.is_some();
    let auto_check = args.check_selection || has_check;
    let auto_enhance = has_enhance;

    let (window_size, resizable, decorations) = if is_popup {
        ((500.0, 600.0), true, true)
    } else {
        ((900.0, 700.0), true, true)
    };

    let flags = AppFlags {
        initial_text,
        auto_check,
        auto_enhance,
        check_clipboard: false,
        enhance_clipboard: false,
    };

    iced::application("Akkurate - 语法助手", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .default_font(Font::with_name("Noto Sans CJK SC"))
        .font(NOTO_SANS_CJK)
        .window_size(window_size)
        .resizable(resizable)
        .decorations(decorations)
        .run_with(|| App::new(flags))
}
