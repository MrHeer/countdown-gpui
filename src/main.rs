use chrono::{DateTime, Days, Duration, Local, NaiveTime};
use gpui::*;
use ui::v_flex;

struct Countdown {
    time: DateTime<Local>,
}

fn format_datetime(datetime: &DateTime<Local>) -> String {
    format!("{}", datetime.format("%H:%M"))
}

fn format_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{:02}:{:02}", hours, minutes)
}

impl Countdown {
    fn get_duration(&self) -> Duration {
        let now = Local::now();
        let mut time = self.time;
        if time < now {
            time = time + Days::new(1);
        }
        time - now
    }

    fn from_hm(hour: u32, min: u32) -> Self {
        let time = Local::now()
            .with_time(NaiveTime::from_hms_opt(hour, min, 0).unwrap())
            .unwrap();
        Self { time }
    }
}

impl Render for Countdown {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .justify_center()
            .items_center()
            .gap_5()
            .size_full()
            .bg(rgba(0xffffff99))
            .child("After")
            .child(
                div()
                    .flex()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .child(format_duration(&self.get_duration())),
            )
            .child("To")
            .child(div().flex().text_2xl().child(format_datetime(&self.time)))
    }
}

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
