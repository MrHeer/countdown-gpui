mod countdown;
mod slider_number;
mod utils;

use clap::Parser;
use countdown::Countdown;
use gpui::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// set hour for timer
    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u32).range(0..24))]
    hour: u32,

    /// set minute for timer
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..60))]
    minute: u32,

    /// set width for window
    #[arg(long, default_value_t = 300.)]
    width: f32,

    /// set height for window
    #[arg(long, default_value_t = 300.)]
    height: f32,

    /// popup window
    #[arg(long, default_value_t = false)]
    popup: bool,
}

fn main() {
    let args = Args::parse();
    let Args {
        hour,
        minute,
        width,
        height,
        popup,
    } = args;

    App::new().run(move |cx: &mut AppContext| {
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        let window_size = size(px(width), px(height));
        let bounds = Bounds::centered(None, window_size, cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                titlebar: None,
                kind: if popup {
                    WindowKind::PopUp
                } else {
                    WindowKind::Normal
                },
                ..Default::default()
            },
            |cx| cx.new_view(|cx| Countdown::new(hour, minute, cx)),
        )
        .unwrap();
    });
}
