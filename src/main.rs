mod countdown;
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
}

fn main() {
    let args = Args::parse();

    App::new().run(move |cx: &mut AppContext| {
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        let window_size = size(px(300.0), px(300.0));
        let bounds = Bounds::centered(None, window_size, cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(window_size),
                titlebar: Some(TitlebarOptions {
                    title: Some("Countdown".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                window_background: WindowBackgroundAppearance::Blurred,
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Countdown::new(args.hour, args.minute)),
        )
        .unwrap();
    });
}
