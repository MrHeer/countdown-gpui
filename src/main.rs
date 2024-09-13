mod countdown;
mod utils;

use countdown::Countdown;
use gpui::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
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
            |cx| cx.new_view(|_cx| Countdown::from_hm(8, 0)),
        )
        .unwrap();
    });
}
